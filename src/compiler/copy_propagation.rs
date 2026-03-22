use std::collections::{HashMap, HashSet};
use crate::compiler::ir::*;

#[derive(Clone)]
enum Value {
    ConstInt(i128),
    ConstBool(bool),
    Temp(usize),
}

fn resolve_temp(temp: &mut Temp, map: &HashMap<usize, Value>) {
    if let Some(val) = map.get(&temp.id) {
        match val {
            Value::Temp(id) => {
                temp.id = *id;
            }
            Value::ConstInt(v) => {
                
            }
            Value::ConstBool(_) => {}
        }
    }
}
pub fn run(ir: &mut IRProgram) {
    for func in &mut ir.functions {
        for block in &mut func.blocks {
            let mut map: HashMap<usize, Value> = HashMap::new();
            let mut vars: HashMap<usize, Value> = HashMap::new();
            let mut used_temps: HashSet<usize> = HashSet::new();

            for instr in &mut block.instructions {
                match instr {

                    // -------------------------
                    // CONST
                    // -------------------------
                    Instr::ConstInt { dst, value } => {
                        map.insert(dst.id, Value::ConstInt(*value));
                    }

                    Instr::ConstBool { dst, value } => {
                        map.insert(dst.id, Value::ConstBool(*value));
                    }
                    Instr::Greater { dst, lhs, rhs } => {
 					   resolve_temp(lhs, &map);
  					  resolve_temp(rhs, &map);
                        
                        let lhs_id = lhs.id;
                        let rhs_id = rhs.id;
                        let dst_clone = dst.clone();

  			  		if let (Some(Value::ConstInt(a)), Some(Value::ConstInt(b))) = (map.get(&lhs_id), map.get(&rhs_id)) {
     		 	 		 let result = a > b;
	
      			 		 *instr = Instr::ConstBool {
           			 		dst: dst_clone.clone(),
          					  value: result,
       					 };

      			  		map.insert(dst_clone.id, Value::ConstBool(result));
  					  } else {
      				 	 map.remove(&dst_clone.id);
  					  }
					}
                    
                    Instr::GreaterEq { dst, lhs, rhs } => {
 					   resolve_temp(lhs, &map);
  					  resolve_temp(rhs, &map);
                        
                        let lhs_id = lhs.id;
                        let rhs_id = rhs.id;
                        let dst_clone = dst.clone();

  			  		if let (Some(Value::ConstInt(a)), Some(Value::ConstInt(b))) = (map.get(&lhs_id), map.get(&rhs_id)) {
     		 	 		 let result = a >= b;
	
      			 		 *instr = Instr::ConstBool {
           			 		dst: dst_clone.clone(),
          					  value: result,
       					 };

      			  		map.insert(dst_clone.id, Value::ConstBool(result));
  					  } else {
      				 	 map.remove(&dst_clone.id);
  					  }
					}
                    
                    Instr::Less { dst, lhs, rhs } => {
 					   resolve_temp(lhs, &map);
  					  resolve_temp(rhs, &map);
                        
                        let lhs_id = lhs.id;
                        let rhs_id = rhs.id;
                        let dst_clone = dst.clone();

  			  		if let (Some(Value::ConstInt(a)), Some(Value::ConstInt(b))) = (map.get(&lhs_id), map.get(&rhs_id)) {
     		 	 		 let result = a < b;
	
      			 		 *instr = Instr::ConstBool {
           			 		dst: dst_clone.clone(),
          					  value: result,
       					 };

      			  		map.insert(dst_clone.id, Value::ConstBool(result));
  					  } else {
      				 	 map.remove(&dst_clone.id);
  					  }
					}
                    
                    Instr::LessEq { dst, lhs, rhs } => {
 					   resolve_temp(lhs, &map);
  					  resolve_temp(rhs, &map);
                        
                        let lhs_id = lhs.id;
                        let rhs_id = rhs.id;
                        let dst_clone = dst.clone();

  			  		if let (Some(Value::ConstInt(a)), Some(Value::ConstInt(b))) = (map.get(&lhs_id), map.get(&rhs_id)) {
     		 	 		 let result = a <= b;
	
      			 		 *instr = Instr::ConstBool {
           			 		dst: dst_clone.clone(),
          					  value: result,
       					 };

      			  		map.insert(dst_clone.id, Value::ConstBool(result));
  					  } else {
      				 	 map.remove(&dst_clone.id);
  					  }
					}

                    // -------------------------
                    // LOAD
                    // -------------------------
                    Instr::Load { dst, id } => {
                        if let Some(val) = vars.get(&id.0) {
    					    map.insert(dst.id, val.clone());
					    } else {
   					     map.remove(&dst.id);
  					  }
                    }
                    // -------------------------
                    // STORE
                    // -------------------------
                    Instr::Store { var, src } => {
                        resolve_temp(src, &map);
                        
                        if let Some(val) = map.get(&src.id){
                            vars.insert(var.0, val.clone());
                        }else{
                            vars.insert(var.0, Value::Temp(src.id));
                        }
                    }

                    // -------------------------
                    // BINÁRIOS
                    // -------------------------
                    Instr::Add { dst, lhs, rhs } => {
                        resolve_temp(lhs, &map);
                        resolve_temp(rhs, &map);

                        // resultado novo → não sabemos ainda
                        map.remove(&dst.id);
                    }

                    // -------------------------
                    // PRINT
                    // -------------------------
                    Instr::Print { temp } => {
                        resolve_temp(temp, &map);
                    }

                    // -------------------------
                    // IF
                    // -------------------------
                    Instr::JumpIfFalse { cond, .. } => {
                        resolve_temp(cond, &map);
                    }

                    // -------------------------
                    // RETURN
                    // -------------------------
                    Instr::Return(Some(temp)) => {
                        resolve_temp(temp, &map);
                    }

                    _ => {}
                }
            }
        }
    }
}
