use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, JsFileSource, JsVariableDeclaration, JsVariableDeclarator};

declare_rule! {
    /// Disallow variables from evolving into `any` type through reassignments.
    ///
    /// In TypeScript, variables without explicit type annotations can evolve their types based on subsequent assignments.
    /// This behavior can accidentally lead to variables with an `any` type, weakening type safety.
    /// Just like the `any` type, evolved `any` types disable many type checking rules and should be avoided to maintain strong type safety.
    /// This rule prevents such cases by ensuring variables do not evolve into `any` type, encouraging explicit type annotations and controlled type evolutions.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// let a;
    /// ````
    ///
    /// ```ts,expect_diagnostic
    /// const b = [];
    /// ````
    ///
    /// ```ts,expect_diagnostic
    /// let c = null;
    /// ````
    ///
    ///
    /// ### Valid
    ///
    /// ```ts
    /// var a = 1;
    /// let a:number;
    /// var b:number;
    /// var b = 10;
    /// let a = 'hello';
    /// const b = ['hello'];
    /// const c = null;
    /// ```
    ///
    pub NoEvolvingAny {
        version: "next",
        name: "noEvolvingAny",
        recommended: true,
    }
}

impl Rule for NoEvolvingAny {
    type Query = Ast<JsVariableDeclaration>;
    type State = JsVariableDeclarator;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let source_type = ctx.source_type::<JsFileSource>().language();
        let is_ts_source = source_type.is_typescript();
        let node = ctx.query();
        let is_declaration = source_type.is_definition_file();

        if is_declaration || !is_ts_source {
            return None;
        }

        for declarator in node.declarators() {
            let variable = declarator.ok()?;

            let is_initialized = variable.initializer().is_some();
            let is_type_annotated = variable.variable_annotation().is_some();

            if !is_initialized && !is_type_annotated {
                return Some(variable);
            }

            if is_initialized {
                let initializer = variable.initializer()?;
                let expression = initializer.expression().ok()?;
                match expression {
                    AnyJsExpression::AnyJsLiteralExpression(literal_expr) => {
                        if literal_expr.as_js_null_literal_expression().is_some()
                            && !is_type_annotated
                        {
                            return Some(variable);
                        }
                    }
                    AnyJsExpression::JsArrayExpression(array_expr) => {
                        if array_expr.elements().into_iter().next().is_none() && !is_type_annotated
                        {
                            return Some(variable);
                        }
                    }
                    _ => continue,
                };
            }
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, node: &Self::State) -> Option<RuleDiagnostic> {
        let variable = node
            .id()
            .ok()?
            .as_any_js_binding()?
            .as_js_identifier_binding()?
            .name_token()
            .ok()?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                variable.text_trimmed_range(),
                markup! {
                    "This variable's type is allowed to evolve implicitly, leading to potential "<Emphasis>"any"</Emphasis>" types."
                },
            )
            .note(markup! {
                "Variable's type may evolve, leading to "<Emphasis>"any"</Emphasis>". Use explicit type or initialization. Specify an explicit type or initial value to avoid implicit type evolution."
            }),
        )
    }
}
