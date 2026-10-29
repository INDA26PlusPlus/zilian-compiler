/*
Codegen takes a AST and gives writes out the executable code
*/

// This is a heavily modified version of the template from:
// github.com/ehsanmok/create-your-own-lang-with-rust/tree/master/secondlang
// A inkwell template for custom rust languages

use std::collections::HashMap;
use std::ptr;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::BasicMetadataTypeEnum;
use inkwell::values::{BasicMetadataValueEnum, IntValue, PointerValue, FunctionValue};
use inkwell::{IntPredicate, OptimizationLevel};

// To understand what's coming from the parser
// Statement can be: Set, Loop, Write
// Expression can be: Num, Var, Add
use crate::parser::{Statement, Expression};


pub enum CompileError {
    TypeError,
    UnknownStatement,
    UnknownExpression,
    UndefinedVariable(String),
}
pub struct CodeGen<'ctx> {
    context: &'ctx Context,
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

        // Compile every statement in the program
        for stmt in &program {
            self.compile_statement(stmt);
        }

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
                let val = self.compile_expression(value)?;
                todo!()
            }

            Statement::Loop { count, content } => {
                // Just add a loop here that compiles the statement val amount of times aka it will loop so many times
                // Alternatively have the program go back and redo the same statements, saving memory
                
                let i = self.compile_expression(count)?;
                let body_bb = self.context.append_basic_block(self.main_fn.unwrap(), "while_body");

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
    pub fn get_module(&self) -> &Module<'ctx> {
        &self.module
    }

    /// Print LLVM IR to string
    pub fn print_ir(&self) -> String {
        self.module.print_to_string().to_string()
    }
} 