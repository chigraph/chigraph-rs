use std::collections::HashMap;

use module::Module;

use graph::DataType;

struct Context {
    modules: HashMap<String, Box<Module>>,
}

impl Context {
    fn new() -> Context {
        Context {
            modules: HashMap::new(),
        }
    }
    
    fn add_module(&mut self, module: Box<Module>) {
        self.modules.insert(module.name(), module)
    }
    
    fn get_type(&self, name: &str) -> Option<&DataType> {
        // parse it into a mod:hello
        
        // find the colon
        let colon = name.find(':')?;
        
        let module_name = name[..colon];
        let type_name = name[colon+1..];
        
        let module = self.modules.get(module_name)?;
        
        let ty = module.node_type(type_name)?;
        
        return Some(ty);
    }
}
