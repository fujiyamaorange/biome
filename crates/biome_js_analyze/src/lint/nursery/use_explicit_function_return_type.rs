use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_semantic::HasClosureAstNode;
use biome_js_syntax::AnyJsBinding;
use biome_js_syntax::{
    AnyJsFunction, JsGetterClassMember, JsGetterObjectMember, JsMethodClassMember,
    JsMethodObjectMember,
};
use biome_rowan::{declare_node_union, AstNode, TextRange};

declare_lint_rule! {
    /// Require explicit return types on functions and class methods.
    ///
    /// Functions in TypeScript often don't need to be given an explicit return type annotation.
    /// Leaving off the return type is less code to read or write and allows the compiler to infer it from the contents of the function.
    ///
    /// However, explicit return types do make it visually more clear what type is returned by a function.
    /// They can also speed up TypeScript type checking performance in large codebases with many large functions.
    /// Explicit return types also reduce the chance of bugs by asserting the return type, and it avoids surprising "action at a distance," where changing the body of one function may cause failures inside another function.
    ///
    /// This rule enforces that functions do have an explicit return type annotation.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// // Should indicate that no value is returned (void)
    /// function test() {
    ///   return;
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// // Should indicate that a number is returned
    /// var fn = function () {
    ///    return 1;
    /// };
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// // Should indicate that a string is returned
    /// var arrowFn = () => 'test';
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// class Test {
    ///   // Should indicate that no value is returned (void)
    ///   method() {
    ///     return;
    ///   }
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// // Should indicate that no value is returned (void)
    /// function test(a: number) {
    ///   a += 1;
    /// }
    /// ```
    ///
    /// ### Valid
    /// ```ts
    /// // No return value should be expected (void)
    /// function test(): void {
    ///   return;
    /// }
    /// ```
    ///
    /// ```ts
    /// // A return value of type number
    /// var fn = function (): number {
    ///   return 1;
    /// }
    /// ```
    ///
    /// ```ts
    /// // A return value of type string
    /// var arrowFn = (): string => 'test';
    /// ```
    ///
    /// ```ts
    /// class Test {
    ///   // No return value should be expected (void)
    ///   method(): void {
    ///     return;
    ///   }
    /// }
    /// ```
    ///
    pub UseExplicitFunctionReturnType {
        version: "next",
        name: "useExplicitFunctionReturnType",
        language: "ts",
        recommended: false,
        sources: &[RuleSource::EslintTypeScript("explicit-function-return-type")],
    }
}

declare_node_union! {
    pub AnyJsFunctionWithReturnType = AnyJsFunction | JsMethodClassMember | JsMethodObjectMember | JsGetterClassMember | JsGetterObjectMember
}

impl Rule for UseExplicitFunctionReturnType {
    type Query = Ast<AnyJsFunctionWithReturnType>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        match node {
            AnyJsFunctionWithReturnType::AnyJsFunction(func) => {
                if func.return_type_annotation().is_some() {
                    return None;
                }

                let func_range = func.syntax().text_range();
                if let Ok(Some(AnyJsBinding::JsIdentifierBinding(id))) = func.id() {
                    return Some(TextRange::new(
                        func_range.start(),
                        id.syntax().text_range().end(),
                    ));
                }

                Some(func_range)
            }
            AnyJsFunctionWithReturnType::JsMethodClassMember(method) => {
                if method.return_type_annotation().is_some() {
                    return None;
                }

                Some(method.node_text_range())
            }
            AnyJsFunctionWithReturnType::JsGetterClassMember(getter) => {
                if getter.return_type().is_some() {
                    return None;
                }

                Some(getter.node_text_range())
            }
            AnyJsFunctionWithReturnType::JsMethodObjectMember(method) => {
                if method.return_type_annotation().is_some() {
                    return None;
                }

                Some(method.node_text_range())
            }
            AnyJsFunctionWithReturnType::JsGetterObjectMember(getter) => {
                if getter.return_type().is_some() {
                    return None;
                }

                Some(getter.node_text_range())
            }
        }
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state,
                markup! {
                    "Missing return type on function."
                },
            )
            .note(markup! {
                "Declaring the return type makes the code self-documenting and can speed up TypeScript type checking."
            })
            .note(markup! {
                "Add a return type annotation."
            }),
        )
    }
}