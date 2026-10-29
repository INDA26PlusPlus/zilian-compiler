/*
Codegen takes a AST and gives writes out the executable code
*/

// This is a heavily modified version of the template from:
// github.com/ehsanmok/create-your-own-lang-with-rust/tree/master/secondlang
// A inkwell template for custom rust languages

// TODO:
/*
Fix Looping
Fix Printing
Fix Actual codegen jit-method
*/

use std::collections::HashMap;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{IntValue, PointerValue, FunctionValue};
use inkwell::{IntPredicate};

// To understand what's coming from the parser
// Statement can be: Set, Loop, Write
// Expression can be: Num, Var, Add
use crate::parser::{Statement, Expression};


#[derive(Debug)]
pub enum CompileError {
    TypeError,
    UnknownStatement,
    UnknownExpression,
    UndefinedVariable(String)
}
pub struct CodeGen<'ctx> {
    // Context is the program
    context: &'ctx Context,
    // Modules are 
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    variables: HashMap<String, PointerValue<'ctx>>,
    // So LLVM needs a function scope to compile, so we'll just keep everything in main
    main_fn: Option<FunctionValue<'ctx>>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {

        let module = context.create_module(module_name);
        let builder = context.create_builder();

        CodeGen {
            context,
            module,
            builder,
            variables: HashMap::new(),
            main_fn: None,
        }
    }

    fn set_main(&mut self, main_function: FunctionValue<'ctx>) {
        self.main_fn = Some(main_function);
    }

    // Look at the program and decide how to compile each statement
    pub fn compile(&mut self, program: Vec<Statement>) -> Result<(), String> {

        // The main function where all things are located in
        let main_function = self.module.add_function(
            "main", 
            self.context.i64_type().fn_type(&[], false), 
            None);
        
        // Set the main function here so we can use it in the compiler
        self.set_main(main_function);

        // Main function block
        let entry = self.context.append_basic_block(main_function, "main_block");
        self.builder.position_at_end(entry);

        // Compile every statement in the program
        for stmt in &program {
            self.compile_statement(stmt).map_err(|e| format!("{:?}", e))?;
        }

        // Close main function and return value (or 0 in case variable is of different name)
        let return_variable_name = "b";
        if self.variables.get(return_variable_name).is_none() {
            self.builder.build_return(Some(&self.context.i64_type().const_zero()));
        }
        else {
            let return_ptr = self.variables.get(return_variable_name).unwrap();
            let return_val = self.builder.build_load(self.context.i64_type(), *return_ptr, "return").unwrap();
            self.builder
                .build_return(Some(&return_val)).unwrap();
        }

        self.module
            .verify()
            .map_err(|e| format!("{}", e.to_string()))?;

        Ok(())
    }

    // Compile Statement
    // Can be Set, Loop, Write
    fn compile_statement(&mut self, stmt: &Statement) -> Result<Option<IntValue<'ctx>>, CompileError> {
        match stmt {
            Statement::Set { name, value } => {
                let val = self.compile_expression(value)?;

                // Check if variable alreadt exists (ptr checks for pointers in memory)
                if let Some(ptr) = self.variables.get(name) {
                    // Found existing variable
                    // Stores the new value to the same existing variable
                    self.builder.build_store(*ptr, val).unwrap();
                } 
                else {
                    // Create new variable
                    let function = self.main_fn.unwrap();
                    let alloca = self.create_entry_block_allocation(&function, name)?;
                    self.builder.build_store(alloca, val).unwrap();
                    self.variables.insert(name.clone(), alloca);
                }

                Ok(Some(val))
            }

            // Will do last, cause not technically needed i think
            Statement::Write { value } => {
                let _val = self.compile_expression(value)?;
                todo!()
            }

            Statement::Loop { count, content } => {
                // Just add a loop here that compiles the statement val amount of times aka it will loop so many times
                // Alternatively have the program go back and redo the same statements, saving memory
                
                // Main function and type
                let function = self.main_fn.unwrap();
                let i64_type = self.context.i64_type(); // Always i64

                // Calculate Count (What our internal counter needs to reach)
                let count_val = self.compile_expression(count)?;

                // Make count variable
                let counter = self.create_entry_block_allocation(&function, "loop_counter")?;
                self.builder.build_store(counter, i64_type.const_zero()).unwrap();

                // The blocks that the loop will be made of
                let cond_bb = self.context.append_basic_block(function, "loop_cond");
                let body_bb = self.context.append_basic_block(function, "loop_body");
                let end_bb = self.context.append_basic_block(function, "loop_end");

                // Goto block and see if we have more to do
                self.builder.build_unconditional_branch(cond_bb).unwrap();

                // Goes into condition block
                self.builder.position_at_end(cond_bb);
                let current_count = self.builder.build_load(
                    i64_type,
                    counter,
                    "counter").unwrap().into_int_value();
                
                // Compares current count with the amount we want
                let loop_condition_checker = self.builder
                    .build_int_compare(
                        IntPredicate::SLT, 
                        current_count, count_val, 
                        "continue")
                    .unwrap();

                // Will run body block until we meet the condition
                self.builder.build_conditional_branch(
                    loop_condition_checker, 
                    body_bb, 
                    end_bb).unwrap();

                // Build main "body" of the loop
                self.builder.position_at_end(body_bb);
                for stmt in content {
                    self.compile_statement(stmt)?;
                }

                // Checks count value
                let current_count = self.builder.build_load(i64_type, counter, "counter").unwrap().into_int_value();
                // Increases count value
                let next = self.builder.build_int_add(current_count, i64_type.const_int(1, false), "next").unwrap();
                // Stores new count int the variable
                self.builder.build_store(counter, next).unwrap();

                // Goes back the condition block
                self.builder.build_unconditional_branch(cond_bb).unwrap();

                // End block under
                self.builder.position_at_end(end_bb);

                Ok(None)
            }

            _ => Err(CompileError::UnknownStatement)
        }
    }

    // Compile Expression
    // Num, Var, Add
    fn compile_expression(&mut self, expr: &Expression) -> Result<IntValue<'ctx>, CompileError> {
        match expr {
            Expression::Num(n) => 
            Ok(self.context.i64_type().const_int(*n as u64, false)),

            Expression::Var(name) => {
                let ptr = self
                    .variables
                    .get(name)
                    .ok_or_else(|| CompileError::UndefinedVariable(name.clone()))?;
                let val = self
                    .builder
                    .build_load(self.context.i64_type(), *ptr, name)
                    .unwrap();
                Ok(val.into_int_value())
            }

            Expression::Add(left, right) => {
                let l = self.compile_expression(left)?;
                let r = self.compile_expression(right)?;

                Ok(self.builder.build_int_add(l, r, "add").unwrap())
            }
        }
    }

    // Create an alloca in the entry block
    fn create_entry_block_allocation(
        &self,
        function: &FunctionValue<'ctx>,
        name: &str,
    ) -> Result<PointerValue<'ctx>, CompileError> {
        let builder = self.context.create_builder();
        let entry = function.get_first_basic_block().unwrap();

        match entry.get_first_instruction() {
            Some(inst) => builder.position_before(&inst),
            None => builder.position_at_end(entry),
        }

        // Everything is an i64
        Ok(builder.build_alloca(self.context.i64_type(), name).unwrap())
    }

    /// Get the compiled module
    pub fn _get_module(&self) -> &Module<'ctx> {
        &self.module
    }

    /// Print LLVM IR to file
    pub fn _print_ir_to_file(&self, path: &str) {
        self.module.print_to_file(path);
    }

    pub fn _print_ir(&self) -> String {
        self.module.print_to_string().to_string()
    }

    // JIT
    pub fn jit_run(codegen: &CodeGen) -> Result<i64, String> {
        
        let engine = codegen
            .module
            .create_jit_execution_engine(inkwell::OptimizationLevel::Default)
            .map_err(|e| format!("Failed to create JIT: {}", e.to_string()))?;

        unsafe  {
            let main: inkwell::execution_engine::JitFunction<unsafe extern "C" fn() -> i64> =
                engine.get_function("main").map_err(|e| e.to_string())?;
            Ok(main.call())
        }
    }

} 