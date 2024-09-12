use rustyline::Editor;
use std::collections::HashMap;

fn main() {
    let scope = &mut HashMap::from([
        (
            "number".to_string(),
            Property::UserDefined(Object {
                properties: HashMap::from([(
                    "number_value".to_string(),
                    Property::BuiltIn(Primitive::Num(0.0)),
                )]),
                methods: HashMap::from([
                    (
                        "+".to_string(),
                        Method::BuiltIn(|args, scope| {
                            if let Property::UserDefined(object) = scope.get("self")?.to_owned() {
                                let mut object = object.clone();
                                object.set_property("number_value".to_string(), {
                                    {
                                        if let Property::BuiltIn(Primitive::Num(i)) =
                                            object.get_property("number_value".to_string())?
                                        {
                                            Property::BuiltIn(Primitive::Num(
                                                i + {
                                                    let Object {
                                                        properties,
                                                        methods: _,
                                                    } = args[0].clone();

                                                    let arg = properties
                                                        .to_owned()
                                                        .get("number_value")?
                                                        .clone();
                                                    if let Property::BuiltIn(Primitive::Num(i)) =
                                                        arg
                                                    {
                                                        i
                                                    } else {
                                                        return None;
                                                    }
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
                            if let Property::UserDefined(object) = scope.get("self")?.to_owned() {
                                let mut object = object.clone();
                                object.set_property("number_value".to_string(), {
                                    {
                                        if let Property::BuiltIn(Primitive::Num(i)) =
                                            object.get_property("number_value".to_string())?
                                        {
                                            Property::BuiltIn(Primitive::Num(
                                                i - {
                                                    let Object {
                                                        properties,
                                                        methods: _,
                                                    } = args[0].clone();

                                                    let arg = properties
                                                        .to_owned()
                                                        .get("number_value")?
                                                        .clone();
                                                    if let Property::BuiltIn(Primitive::Num(i)) =
                                                        arg
                                                    {
                                                        i
                                                    } else {
                                                        return None;
                                                    }
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
                            if let Property::UserDefined(object) = scope.get("self")?.to_owned() {
                                let mut object = object.clone();
                                object.set_property("number_value".to_string(), {
                                    {
                                        if let Property::BuiltIn(Primitive::Num(i)) =
                                            object.get_property("number_value".to_string())?
                                        {
                                            Property::BuiltIn(Primitive::Num(
                                                i * {
                                                    let Object {
                                                        properties,
                                                        methods: _,
                                                    } = args[0].clone();

                                                    let arg = properties
                                                        .to_owned()
                                                        .get("number_value")?
                                                        .clone();
                                                    if let Property::BuiltIn(Primitive::Num(i)) =
                                                        arg
                                                    {
                                                        i
                                                    } else {
                                                        return None;
                                                    }
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
                            if let Property::UserDefined(object) = scope.get("self")?.to_owned() {
                                let mut object = object.clone();
                                object.set_property("number_value".to_string(), {
                                    {
                                        if let Property::BuiltIn(Primitive::Num(i)) =
                                            object.get_property("number_value".to_string())?
                                        {
                                            Property::BuiltIn(Primitive::Num(
                                                i / {
                                                    let Object {
                                                        properties,
                                                        methods: _,
                                                    } = args[0].clone();

                                                    let arg = properties
                                                        .to_owned()
                                                        .get("number_value")?
                                                        .clone();
                                                    if let Property::BuiltIn(Primitive::Num(i)) =
                                                        arg
                                                    {
                                                        i
                                                    } else {
                                                        return None;
                                                    }
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
                        "__display__".to_string(),
                        Method::BuiltIn(|_, scope| {
                            if let Property::UserDefined(object) = scope.get("self")?.to_owned() {
                                if let Property::BuiltIn(Primitive::Num(i)) =
                                    object.get_property("number_value".to_string())?
                                {
                                    Some(Object {
                                        properties: HashMap::from([(
                                            "__display__".to_string(),
                                            Property::BuiltIn(Primitive::Str(i.to_string())),
                                        )]),
                                        methods: HashMap::new(),
                                    })
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
        (
            "string".to_string(),
            Property::UserDefined(Object {
                properties: HashMap::from([(
                    "string_value".to_string(),
                    Property::BuiltIn(Primitive::Str("".to_string())),
                )]),
                methods: HashMap::from([
                    (
                        "+".to_string(),
                        Method::BuiltIn(|args, scope| {
                            if let Property::UserDefined(object) = scope.get("self")?.to_owned() {
                                let mut object = object.clone();
                                object.set_property("string_value".to_string(), {
                                    {
                                        if let Property::BuiltIn(Primitive::Str(s)) =
                                            object.get_property("string_value".to_string())?
                                        {
                                            Property::BuiltIn(Primitive::Str(format!("{}{}", s, {
                                                let Object {
                                                    properties,
                                                    methods: _,
                                                } = args[0].clone();
                                                {
                                                    let arg = properties
                                                        .to_owned()
                                                        .get("string_value")?
                                                        .clone();
                                                    if let Property::BuiltIn(Primitive::Str(i)) =
                                                        arg
                                                    {
                                                        i
                                                    } else {
                                                        return None;
                                                    }
                                                }
                                            },)))
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
                        Method::BuiltIn(|_, scope| {
                            if let Property::UserDefined(object) = scope.get("self")?.to_owned() {
                                if let Property::BuiltIn(Primitive::Str(i)) =
                                    object.get_property("string_value".to_string())?
                                {
                                    println!("{i}");
                                    None
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        }),
                    ),
                    (
                        "__display__".to_string(),
                        Method::BuiltIn(|_, scope| {
                            if let Property::UserDefined(object) = scope.get("self")?.to_owned() {
                                if let Property::BuiltIn(Primitive::Str(i)) =
                                    object.get_property("string_value".to_string())?
                                {
                                    Some(Object {
                                        properties: HashMap::from([(
                                            "__display__".to_string(),
                                            Property::BuiltIn(Primitive::Str(i)),
                                        )]),
                                        methods: HashMap::new(),
                                    })
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
        (
            "bool".to_string(),
            Property::UserDefined(Object {
                properties: HashMap::from([(
                    "bool_value".to_string(),
                    Property::BuiltIn(Primitive::Num(0.0)),
                )]),
                methods: HashMap::from([
                    (
                        "&".to_string(),
                        Method::BuiltIn(|args, scope| {
                            if let Property::UserDefined(object) = scope.get("self")?.to_owned() {
                                let mut object = object.clone();
                                object.set_property("bool_value".to_string(), {
                                    {
                                        if let Property::BuiltIn(Primitive::Num(s)) =
                                            object.get_property("bool_value".to_string())?
                                        {
                                            Property::BuiltIn(Primitive::Num(
                                                if (s != 0.0) && {
                                                    let Object {
                                                        properties,
                                                        methods: _,
                                                    } = args[0].clone();
                                                    {
                                                        let arg = properties
                                                            .to_owned()
                                                            .get("bool_value")?
                                                            .clone();
                                                        if let Property::BuiltIn(Primitive::Num(
                                                            i,
                                                        )) = arg
                                                        {
                                                            i != 0.0
                                                        } else {
                                                            return None;
                                                        }
                                                    }
                                                } {
                                                    1.0
                                                } else {
                                                    0.0
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
                        "|".to_string(),
                        Method::BuiltIn(|args, scope| {
                            if let Property::UserDefined(object) = scope.get("self")?.to_owned() {
                                let mut object = object.clone();
                                object.set_property("bool_value".to_string(), {
                                    {
                                        if let Property::BuiltIn(Primitive::Num(s)) =
                                            object.get_property("bool_value".to_string())?
                                        {
                                            Property::BuiltIn(Primitive::Num(
                                                if (s != 0.0) || {
                                                    let Object {
                                                        properties,
                                                        methods: _,
                                                    } = args[0].clone();
                                                    {
                                                        let arg = properties
                                                            .to_owned()
                                                            .get("bool_value")?
                                                            .clone();
                                                        if let Property::BuiltIn(Primitive::Num(
                                                            i,
                                                        )) = arg
                                                        {
                                                            i != 0.0
                                                        } else {
                                                            return None;
                                                        }
                                                    }
                                                } {
                                                    1.0
                                                } else {
                                                    0.0
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
                        "__display__".to_string(),
                        Method::BuiltIn(|_, scope| {
                            if let Property::UserDefined(object) = scope.get("self")?.to_owned() {
                                if let Property::BuiltIn(Primitive::Num(i)) =
                                    object.get_property("bool_value".to_string())?
                                {
                                    Some(Object {
                                        properties: HashMap::from([(
                                            "__display__".to_string(),
                                            Property::BuiltIn(Primitive::Str(
                                                (i != 0.0).to_string(),
                                            )),
                                        )]),
                                        methods: HashMap::new(),
                                    })
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

    println!("Idletalk 0.1.0");
    let mut rl = Editor::<()>::new();
    let mut n = 0;

    loop {
        let code: String = rl.readline(&format!("[{n}]> ")).unwrap();
        let result = run_program(code, scope);
        if let Some(result) = result {
            println!("{}", result.display(scope.clone()));
        }
        n += 1;
    }
}

fn run_program(source: String, scope: &mut HashMap<String, Property>) -> Option<Object> {
    let mut temp = None;
    for line in tokenize_program(source) {
        if line.len() == 2 {
            if line[0].trim().contains(" ") {
                let header = line[0].split_whitespace().collect::<Vec<&str>>();
                let result = parse_expr(header[0].to_string(), scope.clone())?;
                let object = match result {
                    Program::Expr(i) => i.eval(scope.clone())?,
                    Program::Object(i) => access_variable(&i, scope.to_owned()),
                    Program::Statement(i) => run_program(i.to_string(), &mut scope.clone())?,
                };
                let Object {
                    properties,
                    methods,
                } = object;
                {
                    let mut methods = methods.clone();
                    let mut properties = properties.clone();
                    let result = parse_expr(line[1].trim().to_string(), scope.clone())?;

                    match result {
                        Program::Object(i) => {
                            properties.insert(
                                header[1].to_string(),
                                Property::UserDefined(access_variable(&i, scope.clone())),
                            );
                        }
                        Program::Expr(i) => {
                            methods.insert(
                                header[1].to_string(),
                                Method::UserDefined(Program::Expr(i)),
                            );
                        }
                        Program::Statement(i) => {
                            methods.insert(
                                header[1].to_string(),
                                Method::UserDefined(Program::Statement(i)),
                            );
                        }
                    }
                    temp = Some(Object {
                        properties: properties.clone(),
                        methods: methods.clone(),
                    });
                    scope.insert(header[0].to_string(), Property::UserDefined(temp.clone()?));
                };
            } else {
                temp = Some(
                    match parse_expr(line[1].trim().to_string(), scope.clone())? {
                        Program::Expr(i) => i.eval(scope.clone())?,
                        Program::Object(i) => access_variable(&i, scope.clone()),
                        Program::Statement(i) => run_program(i.to_string(), &mut scope.clone())?,
                    },
                );
                scope.insert(
                    line[0].trim().to_string(),
                    Property::UserDefined(temp.clone()?),
                );
            }
        } else {
            temp = Some(match parse_expr(line.get(0)?.to_string(), scope.clone())? {
                Program::Expr(i) => i.eval(scope.clone())?,
                Program::Object(i) => access_variable(&i, scope.clone()),
                Program::Statement(i) => run_program(i.to_string(), &mut scope.clone())?,
            });
        }
    }
    temp
}

fn parse_object(source: String, scope: HashMap<String, Property>) -> Option<Object> {
    let source = source.trim().to_string();
    if let Ok(i) = source.parse::<f64>() {
        let mut obj = if let Property::UserDefined(obj) = scope.get("number")?.to_owned() {
            obj
        } else {
            return None;
        };
        obj.set_property(
            "number_value".to_string(),
            Property::BuiltIn(Primitive::Num(i)),
        );
        Some(obj.clone())
    } else if let Ok(i) = source.parse::<bool>() {
        let mut obj = if let Property::UserDefined(obj) = scope.get("bool")?.to_owned() {
            obj
        } else {
            return None;
        };
        obj.set_property(
            "bool_value".to_string(),
            Property::BuiltIn(Primitive::Num(if i { 1.0 } else { 0.0 })),
        );
        Some(obj.clone())
    } else if source.starts_with("\"") && source.ends_with("\"") {
        let mut i = source.clone();
        i.remove(i.find("\"")?);
        i.remove(i.rfind("\"")?);
        let mut obj = if let Property::UserDefined(obj) = scope.get("string")?.to_owned() {
            obj
        } else {
            return None;
        };
        obj.set_property(
            "string_value".to_string(),
            Property::BuiltIn(Primitive::Str(i)),
        );
        Some(obj.clone())
    } else {
        Some(Object {
            properties: HashMap::from([(
                "variable".to_string(),
                Property::BuiltIn(Primitive::Str(source)),
            )]),
            methods: HashMap::new(),
        })
    }
}

fn parse_expr(source: String, scope: HashMap<String, Property>) -> Option<Program> {
    let source = source.trim().to_string();
    let tokens = tokenize_expr(source, vec![' ', '\n', '\t', '\r', '　']);
    if tokens.len() >= 3 {
        Some(Program::Expr(Expr {
            object: if tokens.get(0)?.starts_with("(") && tokens.get(0)?.ends_with(")") {
                let mut i = tokens.get(0)?.clone();
                i.remove(i.find("(")?);
                i.remove(i.rfind(")")?);
                let result = parse_expr(i.to_owned().to_string(), scope.clone()).to_owned()?;
                match result {
                    Program::Expr(i) => i.eval(scope.clone())?,
                    Program::Object(i) => access_variable(&i, scope.clone()),
                    Program::Statement(i) => run_program(i.to_string(), &mut scope.clone())?,
                }
            } else if tokens.get(0)?.starts_with("{") && tokens.get(0)?.ends_with("}") {
                let mut i = tokens.get(0)?.clone();
                i.remove(i.find("{")?);
                i.remove(i.rfind("}")?);
                run_program(i.to_string(), &mut scope.clone())?
            } else {
                access_variable(
                    &parse_object(tokens.get(0)?.to_owned(), scope.clone())?,
                    scope.clone(),
                )
            },
            message: tokens.get(1)?.to_string(),
            args: {
                let tokens = tokens.get(2..tokens.len())?.to_vec();
                let mut new_tokens: Vec<Program> = vec![];
                for i in tokens {
                    new_tokens.push(if i.starts_with("(") && i.ends_with(")") {
                        let mut i = i.clone();
                        i.remove(i.find("(")?);
                        i.remove(i.rfind(")")?);
                        parse_expr(i.to_owned().to_string(), scope.clone()).to_owned()?
                    } else if i.starts_with("{") && i.ends_with("}") {
                        let mut i = i.clone();
                        i.remove(i.find("{")?);
                        i.remove(i.rfind("}")?);
                        Program::Statement(i.to_owned().to_string())
                    } else {
                        Program::Object(parse_object(i.to_owned(), scope.clone())?)
                    })
                }
                new_tokens
            },
        }))
    } else if tokens.len() >= 2 {
        Some(Program::Expr(Expr {
            object: if tokens.get(0)?.starts_with("(") && tokens.get(0)?.ends_with(")") {
                let mut i = tokens.get(0)?.clone();
                i.remove(i.find("(")?);
                i.remove(i.rfind(")")?);
                let result = parse_expr(i.to_owned().to_string(), scope.clone()).to_owned()?;
                match result {
                    Program::Expr(i) => i.eval(scope.clone())?,
                    Program::Object(i) => access_variable(&i, scope.clone()),
                    Program::Statement(i) => run_program(i.to_string(), &mut scope.clone())?,
                }
            } else if tokens.get(0)?.starts_with("{") && tokens.get(0)?.ends_with("}") {
                let mut i = tokens.get(0)?.clone();
                i.remove(i.find("{")?);
                i.remove(i.rfind("}")?);
                run_program(i.to_string(), &mut scope.clone())?
            } else {
                access_variable(
                    &parse_object(tokens.get(0)?.to_owned(), scope.clone())?,
                    scope.clone(),
                )
            },
            message: tokens.get(1)?.to_string(),
            args: vec![],
        }))
    } else {
        if tokens.get(0)?.starts_with("(") && tokens.get(0)?.ends_with(")") {
            let mut i = tokens.get(0)?.clone();
            i.remove(i.find("(")?);
            i.remove(i.rfind(")")?);
            let result = parse_expr(i.to_owned().to_string(), scope.clone()).to_owned()?;
            match result {
                Program::Expr(i) => Some(Program::Expr(i)),
                Program::Object(i) => Some(Program::Object(access_variable(&i, scope))),
                Program::Statement(i) => Some(Program::Statement(i)),
            }
        } else if tokens.get(0)?.starts_with("{") && tokens.get(0)?.ends_with("}") {
            let mut i = tokens.get(0)?.clone();
            i.remove(i.find("{")?);
            i.remove(i.rfind("}")?);
            Some(Program::Statement(i))
        } else {
            Some(Program::Object(parse_object(
                tokens[0].to_string(),
                scope.clone(),
            )?))
        }
    }
}

fn tokenize_program(input: String) -> Vec<Vec<String>> {
    let mut tokens: Vec<Vec<String>> = Vec::new();
    let mut current_token = String::new();
    let mut after_equal = String::new();
    let mut is_equal = false;
    let mut in_parentheses: usize = 0;

    for c in input.chars() {
        match c {
            '{' => {
                if is_equal {
                    after_equal.push(c);
                } else {
                    current_token.push(c);
                }
                in_parentheses += 1;
            }
            '}' => {
                if is_equal {
                    after_equal.push(c);
                } else {
                    current_token.push(c);
                }
                in_parentheses -= 1;
            }
            ';' => {
                if in_parentheses != 0 {
                    if is_equal {
                        after_equal.push(c);
                    } else {
                        current_token.push(c);
                    }
                } else {
                    if !current_token.is_empty() {
                        if is_equal {
                            is_equal = false;
                            tokens.push(vec![current_token.clone(), after_equal.clone()]);
                            current_token.clear();
                            after_equal.clear();
                        } else {
                            tokens.push(vec![current_token.clone()]);
                            current_token.clear();
                        }
                    }
                }
            }
            '=' => {
                if in_parentheses != 0 {
                    if is_equal {
                        after_equal.push(c);
                    } else {
                        current_token.push(c);
                    }
                } else {
                    is_equal = true;
                }
            }
            _ => {
                if is_equal {
                    after_equal.push(c);
                } else {
                    current_token.push(c);
                }
            }
        }
    }

    if in_parentheses == 0 && !current_token.is_empty() {
        if is_equal {
            tokens.push(vec![current_token.clone(), after_equal]);
            current_token.clear();
        } else {
            tokens.push(vec![current_token.clone()]);
            current_token.clear();
        }
    }
    tokens
}

fn tokenize_expr(input: String, split: Vec<char>) -> Vec<String> {
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
            '{' if !in_quote => {
                if in_parentheses != 0 {
                    in_parentheses += 1;
                    current_token.push(c);
                } else {
                    in_parentheses += 1;
                    current_token.push(c);
                }
            }
            '}' if !in_quote => {
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
            other => {
                if split.contains(&other) {
                    if in_parentheses != 0 || in_quote {
                        current_token.push(c);
                    } else {
                        if !current_token.is_empty() {
                            tokens.push(current_token.clone());
                            current_token.clear();
                        }
                    }
                } else {
                    current_token.push(c);
                }
            }
        }
    }

    if !(in_parentheses != 0 || in_quote) && !current_token.is_empty() {
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
    UserDefined(Program),
}

impl Method {
    fn eval(&self, args: Vec<Object>, scope: HashMap<String, Property>) -> Option<Object> {
        let args: Vec<Object> = args
            .iter()
            .map(|i| access_variable(i, scope.clone()))
            .collect();
        match self {
            Method::BuiltIn(program) => program(args, scope),
            Method::UserDefined(Program::Expr(expr)) => expr.eval(scope),
            Method::UserDefined(Program::Object(obj)) => Some(obj.to_owned()),
            Method::UserDefined(Program::Statement(stmt)) => {
                run_program(stmt.to_string(), &mut scope.clone())
            }
        }
    }
}

fn access_variable(i: &Object, scope: HashMap<String, Property>) -> Object {
    let Object {
        properties,
        methods: _,
    } = i;

    if let Some(Property::BuiltIn(Primitive::Str(value))) = properties.get("variable") {
        if let Some(Property::UserDefined(j)) = scope.get(&value.clone()) {
            j.to_owned()
        } else {
            i.clone()
        }
    } else {
        i.clone()
    }
}

impl Object {
    pub fn receive_message(
        &self,
        message: String,
        args: Vec<Object>,
        scope: HashMap<String, Property>,
    ) -> Option<Object> {
        let mut scope = scope.clone();
        let binding = access_variable(&self.clone(), scope.clone());
        scope.insert("self".to_string(), Property::UserDefined(binding));
        let binding = access_variable(&self.clone(), scope.clone());
        if let Some(program) = binding.methods.get(message.trim()) {
            scope.extend(self.properties.clone());
            program.eval(args, scope)
        } else {
            if binding.methods.contains_key("__missing__") {
                binding.receive_message("__missing__".to_string(), args, scope)
            } else {
                None
            }
        }
    }

    pub fn get_property(&self, name: String) -> Option<Property> {
        Some(self.properties.get(&name)?.clone())
    }

    pub fn set_property(&mut self, name: String, value: Property) {
        self.properties.insert(name, value);
    }

    pub fn display(&self, scope: HashMap<String, Property>) -> String {
        if let Some(Object {
            properties,
            methods: _,
        }) = self.receive_message("__display__".to_string(), vec![], scope)
        {
            if let Some(Property::BuiltIn(Primitive::Str(i))) = properties.get("__display__") {
                i.to_string()
            } else {
                format!("{self:?}")
            }
        } else {
            format!("{self:?}")
        }
    }
}

#[derive(Clone, Debug)]
enum Program {
    Expr(Expr),
    Object(Object),
    Statement(String),
}

#[derive(Clone, Debug)]
struct Expr {
    object: Object,
    message: String,
    args: Vec<Program>,
}

impl Expr {
    fn eval(&self, scope: HashMap<String, Property>) -> Option<Object> {
        self.clone().object.receive_message(
            self.message.clone(),
            {
                let mut args = vec![];
                for i in self.args.iter() {
                    args.push(match i {
                        Program::Expr(i) => i.eval(scope.clone())?,
                        Program::Object(i) => access_variable(i, scope.clone()),
                        Program::Statement(i) => run_program(i.to_string(), &mut scope.clone())?,
                    })
                }
                args
            },
            scope.clone(),
        )
    }
}
