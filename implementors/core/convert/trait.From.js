(function() {var implementors = {};
implementors["backtrace"] = [{"text":"impl From&lt;Vec&lt;BacktraceFrame&gt;&gt; for Backtrace","synthetic":false,"types":[]}];
implementors["failure"] = [{"text":"impl From&lt;Error&gt; for Box&lt;dyn StdError&gt;","synthetic":false,"types":[]},{"text":"impl From&lt;Error&gt; for Box&lt;dyn StdError + Send + Sync&gt;","synthetic":false,"types":[]},{"text":"impl&lt;D&gt; From&lt;D&gt; for Context&lt;D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;D: Display + Send + Sync + 'static,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;F:&nbsp;Fail&gt; From&lt;F&gt; for Error","synthetic":false,"types":[]}];
implementors["gimli"] = [{"text":"impl&lt;T&gt; From&lt;T&gt; for DebugFrameOffset&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; From&lt;T&gt; for EhFrameOffset&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; From&lt;DebugInfoOffset&lt;T&gt;&gt; for UnitSectionOffset&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; From&lt;DebugTypesOffset&lt;T&gt;&gt; for UnitSectionOffset&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R&gt; From&lt;R&gt; for DebugAddr&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;Reader&gt; From&lt;R&gt; for DebugFrame&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;Reader&gt; From&lt;R&gt; for EhFrameHdr&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;Reader&gt; From&lt;R&gt; for EhFrame&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R&gt; From&lt;R&gt; for DebugAbbrev&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;Reader&gt; From&lt;R&gt; for DebugAranges&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R&gt; From&lt;R&gt; for DebugLine&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R&gt; From&lt;R&gt; for DebugLoc&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R&gt; From&lt;R&gt; for DebugLocLists&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;Reader&gt; From&lt;R&gt; for DebugPubNames&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;Reader&gt; From&lt;R&gt; for DebugPubTypes&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R&gt; From&lt;R&gt; for DebugRanges&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R&gt; From&lt;R&gt; for DebugRngLists&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R&gt; From&lt;R&gt; for DebugStr&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R&gt; From&lt;R&gt; for DebugStrOffsets&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R&gt; From&lt;R&gt; for DebugLineStr&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R&gt; From&lt;R&gt; for DebugInfo&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R&gt; From&lt;R&gt; for DebugTypes&lt;R&gt;","synthetic":false,"types":[]}];
implementors["imgui"] = [{"text":"impl From&lt;usize&gt; for TextureId","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; From&lt;*const T&gt; for TextureId","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; From&lt;*mut T&gt; for TextureId","synthetic":false,"types":[]},{"text":"impl From&lt;String&gt; for ImString","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; From&lt;ImString&gt; for Cow&lt;'a, ImStr&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; From&lt;&amp;'a ImString&gt; for Cow&lt;'a, ImStr&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T:&nbsp;?Sized + AsRef&lt;ImStr&gt;&gt; From&lt;&amp;'a T&gt; for ImString","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; From&lt;&amp;'a ImStr&gt; for Cow&lt;'a, ImStr&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; From&lt;&amp;'a mut [f32; 3]&gt; for EditableColor&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; From&lt;&amp;'a mut [f32; 4]&gt; for EditableColor&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl From&lt;ImColor&gt; for ImU32","synthetic":false,"types":[]},{"text":"impl From&lt;u32&gt; for ImColor","synthetic":false,"types":[]},{"text":"impl From&lt;[f32; 4]&gt; for ImColor","synthetic":false,"types":[]},{"text":"impl From&lt;(f32, f32, f32, f32)&gt; for ImColor","synthetic":false,"types":[]},{"text":"impl From&lt;[f32; 3]&gt; for ImColor","synthetic":false,"types":[]},{"text":"impl From&lt;(f32, f32, f32)&gt; for ImColor","synthetic":false,"types":[]},{"text":"impl From&lt;i32&gt; for Id&lt;'static&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T:&nbsp;?Sized + AsRef&lt;str&gt;&gt; From&lt;&amp;'a T&gt; for Id&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; From&lt;*const T&gt; for Id&lt;'static&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; From&lt;*mut T&gt; for Id&lt;'static&gt;","synthetic":false,"types":[]}];
implementors["imgui_sys"] = [{"text":"impl From&lt;[f32; 2]&gt; for ImVec2","synthetic":false,"types":[]},{"text":"impl From&lt;(f32, f32)&gt; for ImVec2","synthetic":false,"types":[]},{"text":"impl From&lt;[f32; 2]&gt; for ImVec2_Simple","synthetic":false,"types":[]},{"text":"impl From&lt;(f32, f32)&gt; for ImVec2_Simple","synthetic":false,"types":[]},{"text":"impl From&lt;[f32; 4]&gt; for ImVec4","synthetic":false,"types":[]},{"text":"impl From&lt;(f32, f32, f32, f32)&gt; for ImVec4","synthetic":false,"types":[]},{"text":"impl From&lt;[f32; 4]&gt; for ImVec4_Simple","synthetic":false,"types":[]},{"text":"impl From&lt;(f32, f32, f32, f32)&gt; for ImVec4_Simple","synthetic":false,"types":[]}];
implementors["lock_api"] = [{"text":"impl&lt;R:&nbsp;RawMutex, T&gt; From&lt;T&gt; for Mutex&lt;R, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;RawMutex, G:&nbsp;GetThreadId, T&gt; From&lt;T&gt; for ReentrantMutex&lt;R, G, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;RawRwLock, T&gt; From&lt;T&gt; for RwLock&lt;R, T&gt;","synthetic":false,"types":[]}];
implementors["miniz_oxide"] = [{"text":"impl From&lt;MZFlush&gt; for TDEFLFlush","synthetic":false,"types":[]},{"text":"impl From&lt;StreamResult&gt; for MZResult","synthetic":false,"types":[]},{"text":"impl&lt;'_&gt; From&lt;&amp;'_ StreamResult&gt; for MZResult","synthetic":false,"types":[]}];
implementors["object"] = [{"text":"impl&lt;E:&nbsp;Endian&gt; From&lt;Rel32&lt;E&gt;&gt; for Rela32&lt;E&gt;","synthetic":false,"types":[]},{"text":"impl&lt;E:&nbsp;Endian&gt; From&lt;Rel64&lt;E&gt;&gt; for Rela64&lt;E&gt;","synthetic":false,"types":[]}];
implementors["proc_macro2"] = [{"text":"impl From&lt;Span&gt; for Span","synthetic":false,"types":[]},{"text":"impl From&lt;TokenStream&gt; for TokenStream","synthetic":false,"types":[]},{"text":"impl From&lt;TokenStream&gt; for TokenStream","synthetic":false,"types":[]},{"text":"impl From&lt;TokenTree&gt; for TokenStream","synthetic":false,"types":[]},{"text":"impl From&lt;Group&gt; for TokenTree","synthetic":false,"types":[]},{"text":"impl From&lt;Ident&gt; for TokenTree","synthetic":false,"types":[]},{"text":"impl From&lt;Punct&gt; for TokenTree","synthetic":false,"types":[]},{"text":"impl From&lt;Literal&gt; for TokenTree","synthetic":false,"types":[]}];
implementors["smallvec"] = [{"text":"impl&lt;'a, A:&nbsp;Array&gt; From&lt;&amp;'a [&lt;A as Array&gt;::Item]&gt; for SmallVec&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A::Item: Clone,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;A:&nbsp;Array&gt; From&lt;Vec&lt;&lt;A as Array&gt;::Item&gt;&gt; for SmallVec&lt;A&gt;","synthetic":false,"types":[]},{"text":"impl&lt;A:&nbsp;Array&gt; From&lt;A&gt; for SmallVec&lt;A&gt;","synthetic":false,"types":[]}];
implementors["syn"] = [{"text":"impl From&lt;SelfValue&gt; for Ident","synthetic":false,"types":[]},{"text":"impl From&lt;SelfType&gt; for Ident","synthetic":false,"types":[]},{"text":"impl From&lt;Super&gt; for Ident","synthetic":false,"types":[]},{"text":"impl From&lt;Crate&gt; for Ident","synthetic":false,"types":[]},{"text":"impl From&lt;Extern&gt; for Ident","synthetic":false,"types":[]},{"text":"impl From&lt;Underscore&gt; for Ident","synthetic":false,"types":[]},{"text":"impl From&lt;Path&gt; for Meta","synthetic":false,"types":[]},{"text":"impl From&lt;MetaList&gt; for Meta","synthetic":false,"types":[]},{"text":"impl From&lt;MetaNameValue&gt; for Meta","synthetic":false,"types":[]},{"text":"impl From&lt;Meta&gt; for NestedMeta","synthetic":false,"types":[]},{"text":"impl From&lt;Lit&gt; for NestedMeta","synthetic":false,"types":[]},{"text":"impl From&lt;FieldsNamed&gt; for Fields","synthetic":false,"types":[]},{"text":"impl From&lt;FieldsUnnamed&gt; for Fields","synthetic":false,"types":[]},{"text":"impl From&lt;VisPublic&gt; for Visibility","synthetic":false,"types":[]},{"text":"impl From&lt;VisCrate&gt; for Visibility","synthetic":false,"types":[]},{"text":"impl From&lt;VisRestricted&gt; for Visibility","synthetic":false,"types":[]},{"text":"impl From&lt;ExprArray&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprAssign&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprAssignOp&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprAsync&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprAwait&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprBinary&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprBlock&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprBox&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprBreak&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprCall&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprCast&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprClosure&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprContinue&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprField&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprForLoop&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprGroup&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprIf&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprIndex&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprLet&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprLit&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprLoop&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprMacro&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprMatch&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprMethodCall&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprParen&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprPath&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprRange&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprReference&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprRepeat&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprReturn&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprStruct&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprTry&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprTryBlock&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprTuple&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprType&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprUnary&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprUnsafe&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprWhile&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprYield&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;usize&gt; for Index","synthetic":false,"types":[]},{"text":"impl From&lt;TypeParam&gt; for GenericParam","synthetic":false,"types":[]},{"text":"impl From&lt;LifetimeDef&gt; for GenericParam","synthetic":false,"types":[]},{"text":"impl From&lt;ConstParam&gt; for GenericParam","synthetic":false,"types":[]},{"text":"impl From&lt;Ident&gt; for TypeParam","synthetic":false,"types":[]},{"text":"impl From&lt;TraitBound&gt; for TypeParamBound","synthetic":false,"types":[]},{"text":"impl From&lt;Lifetime&gt; for TypeParamBound","synthetic":false,"types":[]},{"text":"impl From&lt;PredicateType&gt; for WherePredicate","synthetic":false,"types":[]},{"text":"impl From&lt;PredicateLifetime&gt; for WherePredicate","synthetic":false,"types":[]},{"text":"impl From&lt;PredicateEq&gt; for WherePredicate","synthetic":false,"types":[]},{"text":"impl From&lt;LitStr&gt; for Lit","synthetic":false,"types":[]},{"text":"impl From&lt;LitByteStr&gt; for Lit","synthetic":false,"types":[]},{"text":"impl From&lt;LitByte&gt; for Lit","synthetic":false,"types":[]},{"text":"impl From&lt;LitChar&gt; for Lit","synthetic":false,"types":[]},{"text":"impl From&lt;LitInt&gt; for Lit","synthetic":false,"types":[]},{"text":"impl From&lt;LitFloat&gt; for Lit","synthetic":false,"types":[]},{"text":"impl From&lt;LitBool&gt; for Lit","synthetic":false,"types":[]},{"text":"impl From&lt;Literal&gt; for LitInt","synthetic":false,"types":[]},{"text":"impl From&lt;Literal&gt; for LitFloat","synthetic":false,"types":[]},{"text":"impl From&lt;DataStruct&gt; for Data","synthetic":false,"types":[]},{"text":"impl From&lt;DataEnum&gt; for Data","synthetic":false,"types":[]},{"text":"impl From&lt;DataUnion&gt; for Data","synthetic":false,"types":[]},{"text":"impl From&lt;TypeArray&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypeBareFn&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypeGroup&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypeImplTrait&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypeInfer&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypeMacro&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypeNever&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypeParen&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypePath&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypePtr&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypeReference&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypeSlice&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypeTraitObject&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypeTuple&gt; for Type","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; From&lt;T&gt; for Path <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Into&lt;PathSegment&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; From&lt;T&gt; for PathSegment <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Into&lt;Ident&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl From&lt;LexError&gt; for Error","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()