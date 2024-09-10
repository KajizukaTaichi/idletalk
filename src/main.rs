use std::collections::HashMap;

fn main() {
    println!("Idletalk 0.0.1");
    let scope = HashMap::from([(
        "integer".to_string(),
        Property::UserDefined(Object {
            properties: HashMap::from([(
                "value".to_string(),
                Property::BuiltIn(Primitive::Int(0)),
            )]),
            methods: HashMap::from([(
                "+".to_string(),
                Method::BuiltIn(|args, scope| {
                    if let Property::UserDefined(object) = scope.get("self").unwrap().to_owned() {
                        let mut object = object.clone();
                        object.set_property("value".to_string(), {
                            {
                                if let Property::BuiltIn(Primitive::Int(i)) =
                                    object.get_property("value".to_string()).unwrap()
                                {
                                    Property::BuiltIn(Primitive::Int(
                                        i + if let Object {
                                            properties,
                                            methods: _,
                                        } = args[0].clone()
                                        {
                                            let arg =
                                                properties.to_owned().get("value").unwrap().clone();
                                            if let Property::BuiltIn(Primitive::Int(i)) = arg {
                                                i
                                            } else {
                                                return None;
                                            }
                                        } else {
                                            return None;
                                        },
                                    ))
                                } else {
                                    return None;
                                }
                            }
                        });
                        Some(object)
                    } else {
                        None
                    }
                }),
            )]),
        }),
    )]);
    let program = parse_expr("1 + 1".to_string(), scope.clone()).unwrap();
    dbg!(&program);
    dbg!(program.eval(scope));
}

fn parse_object(source: String, scope: HashMap<String, Property>) -> Option<Object> {
    if let Ok(i) = source.parse::<i128>() {
        let mut obj = if let Property::UserDefined(obj) = scope.get("integer").unwrap().to_owned() {
            obj
        } else {
            return None;
        };
        obj.set_property("value".to_string(), Property::BuiltIn(Primitive::Int(i)));
        Some(obj.clone())
    } else {
        None
    }
}

fn parse_expr(source: String, scope: HashMap<String, Property>) -> Option<Expr> {
    let tokens = tokenize_expr(source);
    Some(Expr {
        object: parse_object(tokens.get(0)?.to_string(), scope.clone())?,
        message: tokens.get(1)?.to_string(),
        args: {
            let tokens = tokens.get(2..tokens.len())?.to_vec();
            let tokens: Vec<Object> = tokens
                .iter()
                .map(|i| {
                    if i.starts_with("(") && i.ends_with(")") {
                        let mut i = i.clone();
                        i.remove(i.find("(").unwrap());
                        i.remove(i.rfind(")").unwrap());
                        let expr = parse_expr(i.to_owned().to_string(), scope.clone())
                            .to_owned()
                            .unwrap();
                        expr.eval(scope.clone()).unwrap()
                    } else {
                        parse_object(i.to_owned(), scope.clone()).unwrap()
                    }
                })
                .collect();
            tokens
        },
    })
}

fn tokenize_expr(input: String) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut in_parentheses: usize = 0;

    for c in input.chars() {
        match c {
            '(' => {
                if in_parentheses != 0 {
                    in_parentheses += 1;
                    current_token.push(c);
                } else {
                    in_parentheses += 1;
                    current_token.push(c);
                }
            }
            ')' => {
                if in_parentheses != 0 {
                    current_token.push(c);
                    in_parentheses -= 1;
                    if in_parentheses == 0 {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                }
            }
            ' ' | '\n' | '\t' | '\r' | '　' => {
                if in_parentheses != 0 {
                    current_token.push(c);
                } else {
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                }
            }
            _ => {
                current_token.push(c);
            }
        }
    }

    if !(in_parentheses != 0) && !current_token.is_empty() {
        tokens.push(current_token);
    }
    tokens
}

#[derive(Clone, Debug)]
struct Object {
    properties: HashMap<String, Property>,
    methods: HashMap<String, Method>,
}

#[derive(Clone, Debug)]
enum Property {
    BuiltIn(Primitive),
    UserDefined(Object),
}

#[derive(Clone, Debug)]
enum Primitive {
    Int(i128),
    Str(String),
    Float(f64),
}

#[derive(Clone, Debug)]
enum Method {
    BuiltIn(fn(Vec<Object>, HashMap<String, Property>) -> Option<Object>),
    UserDefined(Expr),
}

impl Method {
    fn eval(&self, args: Vec<Object>, scope: HashMap<String, Property>) -> Option<Object> {
        match self {
            Method::BuiltIn(program) => program(args, scope),
            Method::UserDefined(expr) => expr.eval(scope),
        }
    }
}

impl Object {
    pub fn receive_message(
        &self,
        message: String,
        args: Vec<Object>,
        scope: HashMap<String, Property>,
    ) -> Option<Object> {
        let program = self.methods.get(&message)?;
        let mut scope = scope.clone();
        scope.extend(self.properties.clone());
        scope.insert("self".to_string(), Property::UserDefined(self.clone()));
        program.eval(args, scope)
    }

    pub fn get_property(&self, name: String) -> Option<Property> {
        Some(self.properties.get(&name)?.clone())
    }

    pub fn set_property(&mut self, name: String, value: Property) {
        self.properties.insert(name, value);
    }

    pub fn set_method(&mut self, name: String, value: Expr) {
        self.methods.insert(name, Method::UserDefined(value));
    }
}

#[derive(Clone, Debug)]
struct Expr {
    object: Object,
    message: String,
    args: Vec<Object>,
}

impl Expr {
    fn eval(&self, scope: HashMap<String, Property>) -> Option<Object> {
        self.clone()
            .object
            .receive_message(self.message.clone(), self.args.clone(), scope.clone())
    }
}
