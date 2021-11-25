use vrl::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct IsRegex;

impl Function for IsRegex {
    fn identifier(&self) -> &'static str {
        "is_regex"
    }

    fn parameters(&self) -> &'static [Parameter] {
        &[Parameter {
            keyword: "value",
            kind: kind::ANY,
            required: true,
        }]
    }

    fn examples(&self) -> &'static [Example] {
        &[
            Example {
                title: "string",
                source: r#"is_regex("foobar")"#,
                result: Ok("false"),
            },
            Example {
                title: "regex",
                source: r#"is_regex(r'\d+')"#,
                result: Ok("true"),
            },
            Example {
                title: "null",
                source: r#"is_regex(null)"#,
                result: Ok("false"),
            },
        ]
    }

    fn compile(
        &self,
        _state: &state::Compiler,
        _ctx: &FunctionCompileContext,
        mut arguments: ArgumentList,
    ) -> Compiled {
        let value = arguments.required("value");

        Ok(Box::new(IsRegexFn { value }))
    }
}

#[derive(Clone, Debug)]
struct IsRegexFn {
    value: Box<dyn Expression>,
}

impl Expression for IsRegexFn {
    fn resolve(&self, ctx: &mut Context) -> Resolved {
        self.value.resolve(ctx).map(|v| shared_value!(v.is_regex()))
    }

    fn type_def(&self, _: &state::Compiler) -> TypeDef {
        TypeDef::new().infallible().boolean()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    test_function![
        is_regex => IsRegex;

        bytes {
            args: func_args![value: value!("foobar")],
            want: Ok(value!(false)),
            tdef: TypeDef::new().infallible().boolean(),
        }

        regex {
            args: func_args![value: value!(Regex::new(r"\d+").unwrap())],
            want: Ok(value!(true)),
            tdef: TypeDef::new().infallible().boolean(),
        }
    ];
}
