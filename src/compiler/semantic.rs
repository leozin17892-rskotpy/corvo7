use crate::compiler::parser::{Stmt, Expr, VarDecl, FuncDecl, Type, BinOp, Mutability, Assignment, IfStatement, ForLoop};
use std::collections::HashMap;
use crate::compiler::parser::WhileLoop;
use crate::compiler::parser::Loop;

#[derive(Debug, Clone)]
pub struct SemanticAnalyzer {
    scopes: Vec<HashMap<String, SymbolInfo>>, // Pilha de escopos
    functions: HashMap<String, FunctionInfo>,
    current_function: Option<String>,
    loop_depth: usize,
    errors: Vec<SemanticError>,
}

#[derive(Debug, Clone)]
struct SymbolInfo {
    ty: Type,
    mutability: Mutability,
    initialized: bool,
}

#[derive(Debug, Clone)]
struct FunctionInfo {
    params: Vec<(Type, String)>,
    return_type: Option<Type>,
}

#[derive(Debug, Clone)]
pub enum SemanticErrorKind {
    UndeclaredVariable,
    UndeclaredFunction,
    RedeclaredVariable,
    IntegerOverflow,
    IntegerUnderflow,
    TypeMismatch,
    ImmutableAssignment,
    InvalidOperation,
    MissingReturn,
    WrongReturnType,
    ArgumentCountMismatch,
    ArgumentTypeMismatch,
    InvalidStep,
}

#[derive(Debug, Clone)]
pub struct SemanticError {
    kind: SemanticErrorKind,
    message: String,
}

impl std::fmt::Display for SemanticError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Semantic Error: {} ({:?})", self.message, self.kind)
    }
}

pub struct AnalysisResult {
    pub errors: Vec<SemanticError>,
    pub success: bool,
}

impl AnalysisResult {
    pub fn unwrap_or_exit(self) {
        if !self.errors.is_empty() {
            for err in &self.errors {
                eprintln!("{}", err);
            }
            std::process::exit(1);
        }
    }
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()], // Escopo global
            functions: HashMap::new(),
            current_function: None,
            loop_depth: 0,
            errors: Vec::new(),
        }
    }

    pub fn analyze(&mut self, stmts: &[Stmt]) -> AnalysisResult {
        // Primeira passada: registra todas as funções
        for stmt in stmts {
            if let Stmt::FuncDecl(func) = stmt {
                self.register_function(func);
            }
        }

        // Segunda passada: valida o corpo de tudo
        for stmt in stmts {
            self.check_stmt(stmt);
        }
        AnalysisResult {
            success: self.errors.is_empty(),
            errors: self.errors.clone(),
        }
    }

    // ============ GESTÃO DE ESCOPOS ============
    
    fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare_var(&mut self, name: String, ty: Type, mutability: Mutability) {
        if let Some(current_scope) = self.scopes.last_mut() {
            if current_scope.contains_key(&name) {
                self.error(
                    SemanticErrorKind::RedeclaredVariable,
                    format!("Variable '{}' is already declared in this scope", name)
                );
            } else {
                current_scope.insert(name, SymbolInfo { ty, mutability, initialized: true });
            }
        }
    }

    fn lookup_var(&self, name: &str) -> Option<&SymbolInfo> {
        // Busca do escopo mais interno para o mais externo
        for scope in self.scopes.iter().rev() {
            if let Some(info) = scope.get(name) {
                return Some(info);
            }
        }
        None
    }

    fn register_function(&mut self, func: &FuncDecl) {
        if self.functions.contains_key(&func.name) {
            self.error(
                SemanticErrorKind::RedeclaredVariable,
                format!("Function '{}' is already declared", func.name)
            );
        } else {
            self.functions.insert(
                func.name.clone(),
                FunctionInfo {
                    params: func.params.clone(),
                    return_type: func.return_type.clone(),
                }
            );
        }
    }

    // ============ CHECAGEM DE STATEMENTS ============

    fn check_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::VarDecl(var) => self.check_var_decl(var),
            Stmt::FuncDecl(func) => self.check_func_decl(func),
            Stmt::Assignment(assign) => self.check_assignment(assign),
            Stmt::CompoundAssign { target, op, value } => {
                self.check_compound_assign(target, op, value);
            }
            Stmt::IfStatement(if_stmt) => self.check_if(if_stmt),
            Stmt::WhenStatement(_) => (),
            Stmt::ForLoop(for_loop) => self.check_for(for_loop),
            Stmt::Loop(c7_loop) => self.check_loop(c7_loop),
            Stmt::Return(expr) => self.check_return(expr),
            Stmt::Print(exprs) => {
                for expr in exprs {
                    self.check_expr(expr);
                }
            }
            Stmt::Break => {
                if self.loop_depth == 0 {
                    self.error(
                        SemanticErrorKind::InvalidOperation,
                        "break statement outside of loop".to_string()
                    );
                }
            }
            Stmt::Continue => {
                if self.loop_depth == 0 {
                    self.error(
                        SemanticErrorKind::InvalidOperation,
                        "continue statement outside of loop".to_string()
                    );
                }
            }
            Stmt::WhileLoop(while_loop) => self.check_while(while_loop)
        }
    }

    fn check_while(&mut self, while_loop: &WhileLoop) {
   	 // Condição deve ser bool
   	 let cond_type = self.check_expr(&while_loop.cond);
   	 if !matches!(cond_type, Type::Bool) {
     	   self.error(
       	     SemanticErrorKind::TypeMismatch,
        	    format!("While condition must be bool, found {:?}", cond_type)
       	 );
    	}
    
  	  // Checa corpo do while
   	 self.enter_scope();
        self.loop_depth += 1;
    	for stmt in &while_loop.body {
     	   self.check_stmt(stmt);
  	  }
   	 self.exit_scope();
	}
    fn check_var_decl(&mut self, var: &VarDecl) {
  	 let expr_type = self.check_expr(&var.value);
   
   // Verifica se é uma literal inteira que não cabe no tipo declarado
	   if let Some(literal_val) = self.extract_integer_literal(&var.value) {
     	  if !self.integer_fits_in_type(literal_val, &var.ty) {
    	       let (min, max) = self.type_range(&var.ty);
     	      self.error(
             	  SemanticErrorKind::IntegerOverflow,
             	  format!(
              	     "Integer literal {} is out of range for type {:?} (range: {} to {})",
                	   literal_val, var.ty, min, max
             	  )
           	);
           return;
   	    }
 	  }
   
   // Verificação normal de compatibilidade
  	 if !self.types_compatible(&var.ty, &expr_type) {
       	self.error(
           	SemanticErrorKind::TypeMismatch,
           	format!(
            	   "Type mismatch in variable '{}': expected {:?}, found {:?}",
              	 var.name, var.ty, expr_type
          	 )
      	 );
   }

   self.declare_var(var.name.clone(), var.ty.clone(), var.mutability);
	}

	fn extract_integer_literal(&self, expr: &Expr) -> Option<i128> {
 	  match expr {
      	 Expr::IntLiteral(v) => Some(*v as i128),
    	   Expr::UIntLiteral(v) => Some(*v as i128),
    	   Expr::I128Literal(v) => Some(*v),
    	   Expr::U128Literal(v) => Some(*v as i128),
     	  Expr::Int8(v) => Some(*v as i128),
      	 Expr::Int16(v) => Some(*v as i128),
    	   Expr::Int32(v) => Some(*v as i128),
     	  Expr::Int64(v) => Some(*v as i128),
      	 Expr::Int128(v) => Some(*v),
     	  Expr::UInt8(v) => Some(*v as i128),
      	 Expr::UInt16(v) => Some(*v as i128),
    	   Expr::UInt32(v) => Some(*v as i128),
   	    Expr::UInt64(v) => Some(*v as i128),
      	 Expr::UInt128(v) => Some(*v as i128),
     	  Expr::Int(v) => Some(*v as i128),
    	   Expr::UInt(v) => Some(*v as i128),
     	  _ => None,
	   }
	}

	fn integer_fits_in_type(&self, val: i128, ty: &Type) -> bool {
   	match ty {
   	    Type::I8 => val >= i8::MIN as i128 && val <= i8::MAX as i128,
   	    Type::I16 => val >= i16::MIN as i128 && val <= i16::MAX as i128,
   	    Type::I32 => val >= i32::MIN as i128 && val <= i32::MAX as i128,
   	    Type::I64 => val >= i64::MIN as i128 && val <= i64::MAX as i128,
    	   Type::I128 => true,
    	   Type::U8 => val >= 0 && val <= u8::MAX as i128,
    	   Type::U16 => val >= 0 && val <= u16::MAX as i128,
     	  Type::U32 => val >= 0 && val <= u32::MAX as i128,
    	   Type::U64 => val >= 0 && val <= u64::MAX as i128,
   	    Type::U128 => val >= 0,
   	    Type::Int => val >= i64::MIN as i128 && val <= i64::MAX as i128,
  	     Type::UInt => val >= 0 && val <= u64::MAX as i128,
   	    _ => true,
 	  }
	}

	fn type_range(&self, ty: &Type) -> (String, String) {
  	 match ty {
      	 Type::I8 => (i8::MIN.to_string(), i8::MAX.to_string()),
   	    Type::I16 => (i16::MIN.to_string(), i16::MAX.to_string()),
     	  Type::I32 => (i32::MIN.to_string(), i32::MAX.to_string()),
     	  Type::I64 => (i64::MIN.to_string(), i64::MAX.to_string()),
     	  Type::I128 => (i128::MIN.to_string(), i128::MAX.to_string()),
     	  Type::U8 => ("0".to_string(), u8::MAX.to_string()),
     	  Type::U16 => ("0".to_string(), u16::MAX.to_string()),
    	   Type::U32 => ("0".to_string(), u32::MAX.to_string()),
    	   Type::U64 => ("0".to_string(), u64::MAX.to_string()),
    	   Type::U128 => ("0".to_string(), u128::MAX.to_string()),
    	   Type::Int => (i64::MIN.to_string(), i64::MAX.to_string()),
   	    Type::UInt => ("0".to_string(), u64::MAX.to_string()),
    	   _ => ("?".to_string(), "?".to_string()),
	   }
	}

    fn check_assignment(&mut self, assign: &Assignment) {
        // Verifica se a variável existe
        let var_info = match self.lookup_var(&assign.target) {
            Some(info) => info.clone(),
            None => {
                self.error(
                    SemanticErrorKind::UndeclaredVariable,
                    format!("Variable '{}' is not declared", assign.target)
                );
                return;
            }
        };

        // Verifica se é mutável
        if var_info.mutability == Mutability::Const {
            self.error(
                SemanticErrorKind::ImmutableAssignment,
                format!("Cannot assign to const variable '{}'", assign.target)
            );
        }

        // Verifica tipo
        let expr_type = self.check_expr(&assign.value);
        if !self.types_compatible(&var_info.ty, &expr_type) {
            self.error(
                SemanticErrorKind::TypeMismatch,
                format!(
                    "Type mismatch in assignment to '{}': expected {:?}, found {:?}",
                    assign.target, var_info.ty, expr_type
                )
            );
        }
    }

    fn check_compound_assign(&mut self, target: &str, op: &BinOp, value: &Expr) {
        // Similar ao assignment, mas valida também a operação
        let var_info = match self.lookup_var(target) {
            Some(info) => info.clone(),
            None => {
                self.error(
                    SemanticErrorKind::UndeclaredVariable,
                    format!("Variable '{}' is not declared", target)
                );
                return;
            }
        };

        if var_info.mutability == Mutability::Const {
            self.error(
                SemanticErrorKind::ImmutableAssignment,
                format!("Cannot assign to const variable '{}'", target)
            );
        }

        let expr_type = self.check_expr(value);
        
        // Valida que a operação é compatível com os tipos
        if !self.is_valid_binop(&var_info.ty, op, &expr_type) {
            self.error(
                SemanticErrorKind::InvalidOperation,
                format!(
                    "Invalid operation {:?} between {:?} and {:?}",
                    op, var_info.ty, expr_type
                )
            );
        }
    }

    fn check_if(&mut self, if_stmt: &IfStatement) {
        // Condição deve ser bool
        let cond_type = self.check_expr(&if_stmt.condition);
        if !matches!(cond_type, Type::Bool) {
            self.error(
                SemanticErrorKind::TypeMismatch,
                format!("If condition must be bool, found {:?}", cond_type)
            );
        }

        // Checa then branch
        self.enter_scope();
        for stmt in &if_stmt.then_branch {
            self.check_stmt(stmt);
        }
        self.exit_scope();

        // Checa else branch
        if let Some(else_branch) = &if_stmt.else_branch {
            self.enter_scope();
            for stmt in else_branch {
                self.check_stmt(stmt);
            }
            self.exit_scope();
        }
    }

    fn check_loop(&mut self, loop_statement: &Loop){
        self.enter_scope();
        self.loop_depth += 1;
        for stmt in &loop_statement.body{
            self.check_stmt(stmt);
        }
        self.loop_depth -= 1;
        self.exit_scope();
    }
    fn check_for(&mut self, for_loop: &ForLoop) {
        // Start e end devem ser inteiros
        let start_type = self.check_expr(&for_loop.start);
        let step_type;
        if let Some(_step) = &for_loop.step{
            step_type = Some(self.check_expr(&for_loop.step.clone().unwrap()));
        }else{
            step_type = None
        }
        let end_type = self.check_expr(&for_loop.end);

        if !self.is_integer_type(&start_type) {
            self.error(
                SemanticErrorKind::TypeMismatch,
                format!("For loop start must be integer, found {:?}", start_type)
            );
        }
        if let Some(step_expr) = &for_loop.step.clone(){
            if let Expr::IntLiteral(value) = step_expr{
                if *value == 0{
                    self.error(
                        SemanticErrorKind::InvalidStep,
                        "For loop step cannot be 0".to_string()
                    )
                }
            }
        }
        	if step_type.is_some(){
      		  if !self.is_integer_type(&step_type.clone().unwrap()){
       	   	  self.error(
        	        SemanticErrorKind::TypeMismatch,
               	 format!("For loop step must be integer, found {:?}", step_type)
           	 	);
        		}
            }else{
                if !self.is_integer_type(&step_type.clone().unwrap_or(Type::Int)){
                    self.error(
        	        SemanticErrorKind::TypeMismatch,
               	 format!("For loop step must be integer, found {:?}", step_type)
           	 	);
                }
            }

        if !self.is_integer_type(&end_type) {
            self.error(
                SemanticErrorKind::TypeMismatch,
                format!("For loop end must be integer, found {:?}", end_type)
            );
        }

        // Entra em novo escopo e declara a variável do loop
        self.enter_scope();
        self.loop_depth += 1;
        self.declare_var(for_loop.var.clone(), Type::Int, Mutability::Const);
        
        for stmt in &for_loop.body {
            self.check_stmt(stmt);
        }
        
        self.exit_scope();
    }

    fn check_func_decl(&mut self, func: &FuncDecl) {
        self.current_function = Some(func.name.clone());
        
        // Entra em novo escopo para os parâmetros
        self.enter_scope();
        
        // Declara parâmetros
        for (ty, name) in &func.params {
            self.declare_var(name.clone(), ty.clone(), Mutability::Mutable);
        }

        // Checa corpo da função
        let mut has_return = false;
        for stmt in &func.body {
            if matches!(stmt, Stmt::Return(_)) {
                has_return = true;
            }
            self.check_stmt(stmt);
        }

        // Verifica se função não-void tem return
        if func.return_type.is_some() && !matches!(func.return_type, Some(Type::Void)) && !has_return {
            self.error(
                SemanticErrorKind::MissingReturn,
                format!("Function '{}' must return a value", func.name)
            );
        }

        self.exit_scope();
        self.current_function = None;
    }

    fn check_return(&mut self, expr: &Expr) {
        let expr_type = self.check_expr(expr);
        
        if let Some(func_name) = &self.current_function {
            if let Some(func_info) = self.functions.get(func_name) {
                if let Some(expected_type) = &func_info.return_type {
                    if !self.types_compatible(expected_type, &expr_type) {
                        self.error(
                            SemanticErrorKind::WrongReturnType,
                            format!(
                                "Function '{}' expects return type {:?}, found {:?}",
                                func_name, expected_type, expr_type
                            )
                        );
                    }
                }
            }
        }
    }
    fn const_eval(&self, expr: &Expr) -> Option<i128> {
  	  match expr {

    	    Expr::IntLiteral(v) => Some(*v as i128),
            Expr::UIntLiteral(v) => Some(*v as i128),
            Expr::Int8(v) => Some(*v as i128),
     	   Expr::Int16(v) => Some(*v as i128),
      	  Expr::Int32(v) => Some(*v as i128),
      	  Expr::Int64(v) => Some(*v as i128),
     	   Expr::Int128(v) => Some(*v),

     	   Expr::UInt8(v) => Some(*v as i128),
      	  Expr::UInt16(v) => Some(*v as i128),
     	   Expr::UInt32(v) => Some(*v as i128),
    	    Expr::UInt64(v) => Some(*v as i128),
    	    Expr::UInt128(v) => Some(*v as i128),
        

      	  Expr::BinaryOp { left, op, right } => {
       	     let l = self.const_eval(left)?;
      	      let r = self.const_eval(right)?;

         	   match op {
           	     BinOp::Add => Some(l + r),
             	   BinOp::Sub => Some(l - r),
             	   BinOp::Mul => Some(l * r),
             	   BinOp::Div => Some(l / r),
              	  BinOp::Percent => Some(l % r),
              	  _ => None
          	  }
      	  }
     	   _ => None
  	  }
	}

    // ============ CHECAGEM DE EXPRESSÕES ============

    fn check_expr(&mut self, expr: &Expr) -> Type {
    	match expr {
        Expr::IntLiteral(v) => {
   		 if *v >= i8::MIN as i64 && *v <= i8::MAX as i64 {
      		  Type::I8
   		 } else if *v >= i16::MIN as i64 && *v <= i16::MAX as i64 {
     		   Type::I16
   	 	} else if *v >= i32::MIN as i64 && *v <= i32::MAX as i64 {
       		 Type::I32
   		 }else if *v >= i64::MIN as i64 && *v <= i64::MAX as i64{
                Type::I64 
            }else {
     		   Type::I128 // i64
   		 }
		}

		Expr::UIntLiteral(v) => {
   		 if *v <= u8::MAX as u64 {
      		  Type::U8
   		 } else if *v <= u16::MAX as u64 {
     	 	  Type::U16
   		 } else if *v <= u32::MAX as u64 {
       		 Type::U32
   		 }else if *v <= u64::MAX as u64{
                Type::U64
            }else {
      	 	 Type::U128 // u64
    		}
		}
        Expr::I128Literal(_) => Type::I128,
        Expr::U128Literal(_) => Type::U128,
        Expr::BoolLiteral(_) => Type::Bool,
        Expr::StringLiteral(_) => Type::Str,
        Expr::UInt8(_) => Type::U8,
        Expr::UInt16(_) => Type::U16,
        Expr::UInt32(_) => Type::U32,
        Expr::UInt64(_) => Type::U64,
        Expr::UInt128(_) => Type::U128,
        Expr::Int8(_) => Type::I8,
        Expr::Int16(_) => Type::I16,
        Expr::Int32(_) => Type::I32,
        Expr::Int64(_) => Type::I64,
        Expr::Int128(_) => Type::I128,
        Expr::Int(_) => Type::Int,
        Expr::UInt(_) => Type::UInt,
        
        Expr::Ident(name) => {
            match self.lookup_var(name) {
                Some(info) => info.ty.clone(),
                None => {
                    self.error(
                        SemanticErrorKind::UndeclaredVariable,
                        format!("Variable '{}' is not declared", name)
                    );
                    Type::Void // Tipo de erro
                }
            }
        }

        Expr::Call { name, args } => {
            // Clone os dados da função ANTES de fazer qualquer validação
            let func_info = match self.functions.get(name).cloned() {
                Some(info) => info,
                None => {
                    self.error(
                        SemanticErrorKind::UndeclaredFunction,
                        format!("Function '{}' is not declared", name)
                    );
                    return Type::Void;
                }
            };

            // Agora func_info é owned, não há mais borrow de self.functions
            
            // Verifica número de argumentos
            if args.len() != func_info.params.len() {
                self.error(
                    SemanticErrorKind::ArgumentCountMismatch,
                    format!(
                        "Function '{}' expects {} arguments, found {}",
                        name, func_info.params.len(), args.len()
                    )
                );
            }

            // Verifica tipo dos argumentos
            for (i, arg) in args.iter().enumerate() {
                if let Some((expected_type, _)) = func_info.params.get(i) {
                    let arg_type = self.check_expr(arg);
                    if !self.types_compatible(expected_type, &arg_type) {
                        self.error(
                            SemanticErrorKind::ArgumentTypeMismatch,
                            format!(
                                "Argument {} of function '{}': expected {:?}, found {:?}",
                                i + 1, name, expected_type, arg_type
                            )
                        );
                    }
                }
            }

            func_info.return_type.unwrap_or(Type::Void)
        }

        Expr::BinaryOp { left, op, right } => {

  	 	 let lt = self.check_expr(left);
  	 	 let rt = self.check_expr(right);

    		if lt == Type::Void || rt == Type::Void {
      		  return Type::Void;
  		  }

  		  if lt != rt {
     		   self.error(
         		   SemanticErrorKind::InvalidOperation,
         		   format!(
             		   "Invalid binary operation {:?} between {:?} and {:?}",
            		    op, lt, rt
         	 	  )
        		);
     		   return Type::Void;
  		  }

   		 match op {

        // operadores aritméticos
     		   BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Percent => {

       		     if !self.is_integer_type(&lt) {
             		   self.error(
                  		  SemanticErrorKind::InvalidOperation,
                  		  format!("Operator {:?} requires integer types", op)
               		 );
              		  return Type::Void;
        		    }	
        	    lt
        }

        // comparações
     	   BinOp::Greater
      	  | BinOp::Less
     	   | BinOp::GreaterEqual
     	   | BinOp::LessEqual
     	   | BinOp::DoubleEqual
     	   | BinOp::NotEqual => {

          	  if !self.is_integer_type(&lt) {
               	 self.error(
              	      SemanticErrorKind::InvalidOperation,
              	   	   format!("Operator {:?} requires integer types", op)
              	  );
               	 return Type::Void;
           	 }

            Type::Bool
        }

        _ => Type::Void
   	 }
	}

        Expr::Identity { expr, negated: _ } => {
            let ty = self.check_expr(expr);
            if !matches!(ty, Type::Bool) {
                self.error(
                    SemanticErrorKind::TypeMismatch,
                    format!("Identity operator expects bool, found {:?}", ty)
                );
            }
            Type::Bool
        }

        Expr::Vec { values, size: _ } => {
            if values.is_empty() {
                self.error(
                    SemanticErrorKind::InvalidOperation,
                    "Vector cannot be empty".to_string()
                );
                return Type::Void;
            }

            let first_type = self.check_expr(&values[0]);
            
            // Todos os elementos devem ter o mesmo tipo
            for val in values.iter().skip(1) {
                let val_type = self.check_expr(val);
                if !self.types_compatible(&first_type, &val_type) {
                    self.error(
                        SemanticErrorKind::TypeMismatch,
                        format!("Vector elements must have same type: expected {:?}, found {:?}", first_type, val_type)
                    );
                }
            }

            Type::Vec {
                inner: Box::new(first_type),
                size: values.len(),
            }
        }

        Expr::Unknown => Type::Void,
    }
}

// ============ UTILITÁRIOS ============

fn types_compatible(&self, expected: &Type, found: &Type) -> bool {
    // Implementa regras de compatibilidade de tipos
    match (expected, found) {
        (Type::Int, Type::I8 | Type::I16 | Type::I32 | Type::I64 | Type::I128 | Type::Int) => true,
        (Type::UInt, Type::U8 | Type::U16 | Type::U32 | Type::U64 | Type::U128 | Type::UInt) => true,
        _ => expected == found,
    }
}
    fn is_integer_type(&self, ty: &Type) -> bool {
        matches!(
            ty,
            Type::Int | Type::UInt | 
            Type::I8 | Type::I16 | Type::I32 | Type::I64 | Type::I128 |
            Type::U8 | Type::U16 | Type::U32 | Type::U64 | Type::U128
        )
    }

    fn is_valid_binop(&self, left: &Type, op: &BinOp, right: &Type) -> bool {
        match op {
            BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Percent => {
                self.is_integer_type(left) && self.is_integer_type(right)
            }
            BinOp::DoubleEqual | BinOp::NotEqual | BinOp::Less | BinOp::Greater | BinOp::LessEqual | BinOp::GreaterEqual => {
                self.types_compatible(left, right)
            }
            BinOp::IndentityOp => {
                matches!(left, Type::Bool) && matches!(right, Type::Bool)
            }
            BinOp::CompoundAdd | BinOp::CompoundSub | BinOp::CompoundMul | BinOp::CompoundDiv => {
                self.is_integer_type(left) && self.is_integer_type(right)
            }
        }
    }

    fn result_type_of_binop(&self, left: &Type, op: &BinOp, _right: &Type) -> Type {
        match op {
            BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Percent => left.clone(),
            BinOp::DoubleEqual | BinOp::NotEqual | BinOp::Less | BinOp::Greater | BinOp::LessEqual | BinOp::GreaterEqual | BinOp::IndentityOp => Type::Bool,
            BinOp::CompoundAdd | BinOp::CompoundSub | BinOp::CompoundMul | BinOp::CompoundDiv => left.clone(),
        }
    }

    fn error(&mut self, kind: SemanticErrorKind, message: String) {
        self.errors.push(SemanticError { kind, message });
    }
}    