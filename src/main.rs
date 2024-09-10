use rustyline::Editor;
use std::collections::HashMap;

#[allow(warnings)]
fn main() {
    let scope = &mut HashMap::from([
        (
            "number".to_string(),
            Property::UserDefined(Object {
                properties: HashMap::from([(
                    "value".to_string(),
                    Property::BuiltIn(Primitive::Num(0.0)),
                )]),
                methods: HashMap::from([
                    (
                        "+".to_string(),
                        Method::BuiltIn(|args, scope| {
                            if let Property::UserDefined(object) =
                                scope.get("self").unwrap().to_owned()
                            {
                                let mut object = object.clone();
                                object.set_property("value".to_string(), {
                                    {
                                        if let Property::BuiltIn(Primitive::Num(i)) =
                                            object.get_property("value".to_string()).unwrap()
                                        {
                                            Property::BuiltIn(Primitive::Num(
                                                i + if let Object {
                                                    properties,
                                                    methods: _,
                                                } = args[0].clone()
                                                {
                                                    let arg = properties
                                                        .to_owned()
                                                        .get("value")
                                                        .unwrap()
                                                        .clone();
                                                    if let Property::BuiltIn(Primitive::Num(i)) =
                                                        arg
                                                    {
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
                    ),
                    (
                        "-".to_string(),
                        Method::BuiltIn(|args, scope| {
                            if let Property::UserDefined(object) =
                                scope.get("self").unwrap().to_owned()
                            {
                                let mut object = object.clone();
                                object.set_property("value".to_string(), {
                                    {
                                        if let Property::BuiltIn(Primitive::Num(i)) =
                                            object.get_property("value".to_string()).unwrap()
                                        {
                                            Property::BuiltIn(Primitive::Num(
                                                i - if let Object {
                                                    properties,
                                                    methods: _,
                                                } = args[0].clone()
                                                {
                                                    let arg = properties
                                                        .to_owned()
                                                        .get("value")
                                                        .unwrap()
                                                        .clone();
                                                    if let Property::BuiltIn(Primitive::Num(i)) =
                                                        arg
                                                    {
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
                    ),
                    (
                        "*".to_string(),
                        Method::BuiltIn(|args, scope| {
                            if let Property::UserDefined(object) =
                                scope.get("self").unwrap().to_owned()
                            {
                                let mut object = object.clone();
                                object.set_property("value".to_string(), {
                                    {
                                        if let Property::BuiltIn(Primitive::Num(i)) =
                                            object.get_property("value".to_string()).unwrap()
                                        {
                                            Property::BuiltIn(Primitive::Num(
                                                i * if let Object {
                                                    properties,
                                                    methods: _,
                                                } = args[0].clone()
                                                {
                                                    let arg = properties
                                                        .to_owned()
                                                        .get("value")
                                                        .unwrap()
                                                        .clone();
                                                    if let Property::BuiltIn(Primitive::Num(i)) =
                                                        arg
                                                    {
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
                    ),
                    (
                        "/".to_string(),
                        Method::BuiltIn(|args, scope| {
                            if let Property::UserDefined(object) =
                                scope.get("self").unwrap().to_owned()
                            {
                                let mut object = object.clone();
                                object.set_property("value".to_string(), {
                                    {
                                        if let Property::BuiltIn(Primitive::Num(i)) =
                                            object.get_property("value".to_string()).unwrap()
                                        {
                                            Property::BuiltIn(Primitive::Num(
                                                i / if let Object {
                                                    properties,
                                                    methods: _,
                                                } = args[0].clone()
                                                {
                                                    let arg = properties
                                                        .to_owned()
                                                        .get("value")
                                                        .unwrap()
                                                        .clone();
                                                    if let Property::BuiltIn(Primitive::Num(i)) =
                                                        arg
                                                    {
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
                    ),
                ]),
            }),
        ),
        (
            "string".to_string(),
            Property::UserDefined(Object {
                properties: HashMap::from([(
                    "value".to_string(),
                    Property::BuiltIn(Primitive::Str("".to_string())),
                )]),
                methods: HashMap::from([
                    (
                        "concat".to_string(),
                        Method::BuiltIn(|args, scope| {
                            if let Property::UserDefined(object) =
                                scope.get("self").unwrap().to_owned()
                            {
                                let mut object = object.clone();
                                object.set_property("value".to_string(), {
                                    {
                                        if let Property::BuiltIn(Primitive::Str(s)) =
                                            object.get_property("value".to_string()).unwrap()
                                        {
                                            Property::BuiltIn(Primitive::Str(format!(
                                                "{}{}",
                                                s,
                                                if let Object {
                                                    properties,
                                                    methods: _,
                                                } = args[0].clone()
                                                {
                                                    let arg = properties
                                                        .to_owned()
                                                        .get("value")
                                                        .unwrap()
                                                        .clone();
                                                    if let Property::BuiltIn(Primitive::Str(i)) =
                                                        arg
                                                    {
                                                        i
                                                    } else {
                                                        return None;
                                                    }
                                                } else {
                                                    return None;
                                                },
                                            )))
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
                    ),
                    (
                        "print".to_string(),
                        Method::BuiltIn(|args, scope| {
                            if let Property::UserDefined(object) =
                                scope.get("self").unwrap().to_owned()
                            {
                                if let Property::BuiltIn(Primitive::Str(i)) =
                                    object.get_property("value".to_string()).unwrap()
                                {
                                    println!("{i}");
                                    Some(object)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        }),
                    ),
                ]),
            }),
        ),
    ]);

    println!("Idletalk 0.0.1");
    let mut rl = Editor::<()>::new();
    let mut n = 0;

    loop {
        let code: String = rl.readline(&format!("[{n}]> ")).unwrap();
        let result = run_program(code, scope);
        dbg!(result);
        n += 1;
    }
}

fn run_program(source: String, scope: &mut HashMap<String, Property>) -> Option<Object> {
    let mut temp = None;
    for line in source.split(";") {
        if line.contains(":=") {
            let line: Vec<&str> = line.split(":=").collect();
            scope.insert(
                line[0].trim().to_string(),
                Property::UserDefined(eval_expr(line[1].trim().to_string(), scope.clone())?),
            );
        } else {
            temp = eval_expr(line.to_string(), scope.clone());
        }
    }
    temp
}

fn parse_object(source: String, scope: HashMap<String, Property>) -> Option<Object> {
    let source = source.trim().to_string();
    if let Some(Property::UserDefined(i)) = scope.get(&source) {
        Some(i.to_owned())
    } else if let Ok(i) = source.parse::<f64>() {
        let mut obj = if let Property::UserDefined(obj) = scope.get("number").unwrap().to_owned() {
            obj
        } else {
            return None;
        };
        obj.set_property("value".to_string(), Property::BuiltIn(Primitive::Num(i)));
        Some(obj.clone())
    } else if source.starts_with("\"") && source.ends_with("\"") {
        let mut i = source.clone();
        i.remove(i.find("\"").unwrap());
        i.remove(i.rfind("\"").unwrap());
        let mut obj = if let Property::UserDefined(obj) = scope.get("string").unwrap().to_owned() {
            obj
        } else {
            return None;
        };
        obj.set_property("value".to_string(), Property::BuiltIn(Primitive::Str(i)));
        Some(obj.clone())
    } else {
        None
    }
}

fn eval_expr(source: String, scope: HashMap<String, Property>) -> Option<Object> {
    let tokens = tokenize_expr(source);
    if tokens.len() >= 3 {
        Expr {
            object: if tokens.get(0)?.starts_with("(") && tokens.get(0)?.ends_with(")") {
                let mut i = tokens.get(0)?.clone();
                i.remove(i.find("(").unwrap());
                i.remove(i.rfind(")").unwrap());
                eval_expr(i.to_owned().to_string(), scope.clone())
                    .to_owned()
                    .unwrap()
            } else {
                parse_object(tokens.get(0)?.to_owned(), scope.clone()).unwrap()
            },
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
                            eval_expr(i.to_owned().to_string(), scope.clone())
                                .to_owned()
                                .unwrap()
                        } else {
                            parse_object(i.to_owned(), scope.clone()).unwrap()
                        }
                    })
                    .collect();
                tokens
            },
        }
        .eval(scope)
    } else {
        parse_object(tokens.get(0)?.clone(), scope)
    }
}

fn tokenize_expr(input: String) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut in_parentheses: usize = 0;
    let mut in_quote = false;

    for c in input.chars() {
        match c {
            '(' if !in_quote => {
                if in_parentheses != 0 {
                    in_parentheses += 1;
                    current_token.push(c);
                } else {
                    in_parentheses += 1;
                    current_token.push(c);
                }
            }
            ')' if !in_quote => {
                if in_parentheses != 0 {
                    current_token.push(c);
                    in_parentheses -= 1;
                    if in_parentheses == 0 {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                }
            }
            '"' => {
                in_quote = !in_quote;
                current_token.push(c);
            }
            ' ' | '\n' | '\t' | '\r' | '　' => {
                if in_parentheses != 0 && !in_quote {
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
    Num(f64),
    Str(String),
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
