(function() {var type_impls = {
"glib":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Class%3CT%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/glib/object.rs.html#3853-3975\">source</a><a href=\"#impl-Class%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T: <a class=\"trait\" href=\"glib/object/trait.IsClass.html\" title=\"trait glib::object::IsClass\">IsClass</a>&gt; <a class=\"struct\" href=\"glib/object/struct.Class.html\" title=\"struct glib::object::Class\">Class</a>&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.type_\" class=\"method\"><a class=\"src rightside\" href=\"src/glib/object.rs.html#3861-3868\">source</a><h4 class=\"code-header\">pub fn <a href=\"glib/object/struct.Class.html#tymethod.type_\" class=\"fn\">type_</a>(&amp;self) -&gt; <a class=\"struct\" href=\"glib/types/struct.Type.html\" title=\"struct glib::types::Type\">Type</a></h4></section></summary><div class=\"docblock\"><p>Get the type id for this class.</p>\n<p>This is not equivalent to <code>T::static_type()</code> but is the type of the subclass of <code>T</code> where\nthis class belongs to.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.upcast_ref\" class=\"method\"><a class=\"src rightside\" href=\"src/glib/object.rs.html#3873-3881\">source</a><h4 class=\"code-header\">pub fn <a href=\"glib/object/struct.Class.html#tymethod.upcast_ref\" class=\"fn\">upcast_ref</a>&lt;U: <a class=\"trait\" href=\"glib/object/trait.IsClass.html\" title=\"trait glib::object::IsClass\">IsClass</a>&gt;(&amp;self) -&gt; &amp;<a class=\"struct\" href=\"glib/object/struct.Class.html\" title=\"struct glib::object::Class\">Class</a>&lt;U&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"glib/object/trait.IsA.html\" title=\"trait glib::object::IsA\">IsA</a>&lt;U&gt;,</div></h4></section></summary><div class=\"docblock\"><p>Casts this class to a reference to a parent type’s class.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.upcast_ref_mut\" class=\"method\"><a class=\"src rightside\" href=\"src/glib/object.rs.html#3886-3894\">source</a><h4 class=\"code-header\">pub fn <a href=\"glib/object/struct.Class.html#tymethod.upcast_ref_mut\" class=\"fn\">upcast_ref_mut</a>&lt;U: <a class=\"trait\" href=\"glib/object/trait.IsClass.html\" title=\"trait glib::object::IsClass\">IsClass</a>&gt;(&amp;mut self) -&gt; &amp;mut <a class=\"struct\" href=\"glib/object/struct.Class.html\" title=\"struct glib::object::Class\">Class</a>&lt;U&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"glib/object/trait.IsA.html\" title=\"trait glib::object::IsA\">IsA</a>&lt;U&gt;,</div></h4></section></summary><div class=\"docblock\"><p>Casts this class to a mutable reference to a parent type’s class.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.downcast_ref\" class=\"method\"><a class=\"src rightside\" href=\"src/glib/object.rs.html#3900-3912\">source</a><h4 class=\"code-header\">pub fn <a href=\"glib/object/struct.Class.html#tymethod.downcast_ref\" class=\"fn\">downcast_ref</a>&lt;U&gt;(&amp;self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;&amp;<a class=\"struct\" href=\"glib/object/struct.Class.html\" title=\"struct glib::object::Class\">Class</a>&lt;U&gt;&gt;<div class=\"where\">where\n    U: <a class=\"trait\" href=\"glib/object/trait.IsA.html\" title=\"trait glib::object::IsA\">IsA</a>&lt;T&gt; + <a class=\"trait\" href=\"glib/object/trait.IsClass.html\" title=\"trait glib::object::IsClass\">IsClass</a>,</div></h4></section></summary><div class=\"docblock\"><p>Casts this class to a reference to a child type’s class or\nfails if this class is not implementing the child class.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.downcast_ref_mut\" class=\"method\"><a class=\"src rightside\" href=\"src/glib/object.rs.html#3918-3930\">source</a><h4 class=\"code-header\">pub fn <a href=\"glib/object/struct.Class.html#tymethod.downcast_ref_mut\" class=\"fn\">downcast_ref_mut</a>&lt;U&gt;(&amp;mut self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;&amp;mut <a class=\"struct\" href=\"glib/object/struct.Class.html\" title=\"struct glib::object::Class\">Class</a>&lt;U&gt;&gt;<div class=\"where\">where\n    U: <a class=\"trait\" href=\"glib/object/trait.IsA.html\" title=\"trait glib::object::IsA\">IsA</a>&lt;T&gt; + <a class=\"trait\" href=\"glib/object/trait.IsClass.html\" title=\"trait glib::object::IsClass\">IsClass</a>,</div></h4></section></summary><div class=\"docblock\"><p>Casts this class to a mutable reference to a child type’s class or\nfails if this class is not implementing the child class.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_type\" class=\"method\"><a class=\"src rightside\" href=\"src/glib/object.rs.html#3938-3955\">source</a><h4 class=\"code-header\">pub fn <a href=\"glib/object/struct.Class.html#tymethod.from_type\" class=\"fn\">from_type</a>(type_: <a class=\"struct\" href=\"glib/types/struct.Type.html\" title=\"struct glib::types::Type\">Type</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"struct\" href=\"glib/object/struct.ClassRef.html\" title=\"struct glib::object::ClassRef\">ClassRef</a>&lt;'static, T&gt;&gt;</h4></section></summary><div class=\"docblock\"><p>Gets the class struct for <code>Self</code> of <code>type_</code>.</p>\n<p>This will return <code>None</code> if <code>type_</code> is not a subclass of <code>Self</code>.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.parent\" class=\"method\"><a class=\"src rightside\" href=\"src/glib/object.rs.html#3961-3974\">source</a><h4 class=\"code-header\">pub fn <a href=\"glib/object/struct.Class.html#tymethod.parent\" class=\"fn\">parent</a>(&amp;self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"struct\" href=\"glib/object/struct.ClassRef.html\" title=\"struct glib::object::ClassRef\">ClassRef</a>&lt;'_, T&gt;&gt;</h4></section></summary><div class=\"docblock\"><p>Gets the parent class struct, if any.</p>\n</div></details></div></details>",0,"glib::object::ObjectClass"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-DerefMut-for-Class%3CT%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/glib/object.rs.html#4046-4054\">source</a><a href=\"#impl-DerefMut-for-Class%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T: <a class=\"trait\" href=\"glib/object/trait.ParentClassIs.html\" title=\"trait glib::object::ParentClassIs\">ParentClassIs</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html\" title=\"trait core::ops::deref::DerefMut\">DerefMut</a> for <a class=\"struct\" href=\"glib/object/struct.Class.html\" title=\"struct glib::object::Class\">Class</a>&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.deref_mut\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/glib/object.rs.html#4048-4053\">source</a><a href=\"#method.deref_mut\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html#tymethod.deref_mut\" class=\"fn\">deref_mut</a>(&amp;mut self) -&gt; &amp;mut Self::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target\" title=\"type core::ops::deref::Deref::Target\">Target</a></h4></section></summary><div class='docblock'>Mutably dereferences the value.</div></details></div></details>","DerefMut","glib::object::ObjectClass"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Deref-for-Class%3CT%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/glib/object.rs.html#4034-4044\">source</a><a href=\"#impl-Deref-for-Class%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T: <a class=\"trait\" href=\"glib/object/trait.ParentClassIs.html\" title=\"trait glib::object::ParentClassIs\">ParentClassIs</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"glib/object/struct.Class.html\" title=\"struct glib::object::Class\">Class</a>&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Target\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Target\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target\" class=\"associatedtype\">Target</a> = <a class=\"struct\" href=\"glib/object/struct.Class.html\" title=\"struct glib::object::Class\">Class</a>&lt;&lt;T as <a class=\"trait\" href=\"glib/object/trait.ParentClassIs.html\" title=\"trait glib::object::ParentClassIs\">ParentClassIs</a>&gt;::<a class=\"associatedtype\" href=\"glib/object/trait.ParentClassIs.html#associatedtype.Parent\" title=\"type glib::object::ParentClassIs::Parent\">Parent</a>&gt;</h4></section></summary><div class='docblock'>The resulting type after dereferencing.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.deref\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/glib/object.rs.html#4038-4043\">source</a><a href=\"#method.deref\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref\" class=\"fn\">deref</a>(&amp;self) -&gt; &amp;Self::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target\" title=\"type core::ops::deref::Deref::Target\">Target</a></h4></section></summary><div class='docblock'>Dereferences the value.</div></details></div></details>","Deref","glib::object::ObjectClass"],["<section id=\"impl-Send-for-Class%3CT%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/glib/object.rs.html#3977\">source</a><a href=\"#impl-Send-for-Class%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T: <a class=\"trait\" href=\"glib/object/trait.IsClass.html\" title=\"trait glib::object::IsClass\">IsClass</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glib/object/struct.Class.html\" title=\"struct glib::object::Class\">Class</a>&lt;T&gt;</h3></section>","Send","glib::object::ObjectClass"],["<section id=\"impl-Sync-for-Class%3CT%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/glib/object.rs.html#3978\">source</a><a href=\"#impl-Sync-for-Class%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T: <a class=\"trait\" href=\"glib/object/trait.IsClass.html\" title=\"trait glib::object::IsClass\">IsClass</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"glib/object/struct.Class.html\" title=\"struct glib::object::Class\">Class</a>&lt;T&gt;</h3></section>","Sync","glib::object::ObjectClass"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-AsRef%3C%3CT+as+ObjectType%3E::GlibClassType%3E-for-Class%3CT%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/glib/object.rs.html#3980-3985\">source</a><a href=\"#impl-AsRef%3C%3CT+as+ObjectType%3E::GlibClassType%3E-for-Class%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T: <a class=\"trait\" href=\"glib/object/trait.IsClass.html\" title=\"trait glib::object::IsClass\">IsClass</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;&lt;T as <a class=\"trait\" href=\"glib/object/trait.ObjectType.html\" title=\"trait glib::object::ObjectType\">ObjectType</a>&gt;::<a class=\"associatedtype\" href=\"glib/object/trait.ObjectType.html#associatedtype.GlibClassType\" title=\"type glib::object::ObjectType::GlibClassType\">GlibClassType</a>&gt; for <a class=\"struct\" href=\"glib/object/struct.Class.html\" title=\"struct glib::object::Class\">Class</a>&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.as_ref\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/glib/object.rs.html#3982-3984\">source</a><a href=\"#method.as_ref\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref\" class=\"fn\">as_ref</a>(&amp;self) -&gt; &amp;T::<a class=\"associatedtype\" href=\"glib/object/trait.ObjectType.html#associatedtype.GlibClassType\" title=\"type glib::object::ObjectType::GlibClassType\">GlibClassType</a></h4></section></summary><div class='docblock'>Converts this type into a shared reference of the (usually inferred) input type.</div></details></div></details>","AsRef<<T as ObjectType>::GlibClassType>","glib::object::ObjectClass"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-ObjectClassSubclassExt-for-Class%3CObject%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/glib/subclass/object.rs.html#220\">source</a><a href=\"#impl-ObjectClassSubclassExt-for-Class%3CObject%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"glib/subclass/object/trait.ObjectClassSubclassExt.html\" title=\"trait glib::subclass::object::ObjectClassSubclassExt\">ObjectClassSubclassExt</a> for <a class=\"struct\" href=\"glib/object/struct.Class.html\" title=\"struct glib::object::Class\">Class</a>&lt;<a class=\"struct\" href=\"glib/object/struct.Object.html\" title=\"struct glib::object::Object\">Object</a>&gt;</h3></section></summary><div class=\"impl-items\"><section id=\"method.override_signal_class_handler\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/glib/subclass/object.rs.html#206-217\">source</a><a href=\"#method.override_signal_class_handler\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"glib/subclass/object/trait.ObjectClassSubclassExt.html#method.override_signal_class_handler\" class=\"fn\">override_signal_class_handler</a>&lt;F&gt;(&amp;mut self, name: &amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>, class_handler: F)<div class=\"where\">where\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html\" title=\"trait core::ops::function::Fn\">Fn</a>(&amp;<a class=\"struct\" href=\"glib/subclass/signal/struct.SignalClassHandlerToken.html\" title=\"struct glib::subclass::signal::SignalClassHandlerToken\">SignalClassHandlerToken</a>, &amp;[<a class=\"struct\" href=\"glib/value/struct.Value.html\" title=\"struct glib::value::Value\">Value</a>]) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"struct\" href=\"glib/value/struct.Value.html\" title=\"struct glib::value::Value\">Value</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,</div></h4></section></div></details>","ObjectClassSubclassExt","glib::object::ObjectClass"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-AsMut%3C%3CT+as+ObjectType%3E::GlibClassType%3E-for-Class%3CT%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/glib/object.rs.html#3987-3992\">source</a><a href=\"#impl-AsMut%3C%3CT+as+ObjectType%3E::GlibClassType%3E-for-Class%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T: <a class=\"trait\" href=\"glib/object/trait.IsClass.html\" title=\"trait glib::object::IsClass\">IsClass</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html\" title=\"trait core::convert::AsMut\">AsMut</a>&lt;&lt;T as <a class=\"trait\" href=\"glib/object/trait.ObjectType.html\" title=\"trait glib::object::ObjectType\">ObjectType</a>&gt;::<a class=\"associatedtype\" href=\"glib/object/trait.ObjectType.html#associatedtype.GlibClassType\" title=\"type glib::object::ObjectType::GlibClassType\">GlibClassType</a>&gt; for <a class=\"struct\" href=\"glib/object/struct.Class.html\" title=\"struct glib::object::Class\">Class</a>&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.as_mut\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/glib/object.rs.html#3989-3991\">source</a><a href=\"#method.as_mut\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html#tymethod.as_mut\" class=\"fn\">as_mut</a>(&amp;mut self) -&gt; &amp;mut T::<a class=\"associatedtype\" href=\"glib/object/trait.ObjectType.html#associatedtype.GlibClassType\" title=\"type glib::object::ObjectType::GlibClassType\">GlibClassType</a></h4></section></summary><div class='docblock'>Converts this type into a mutable reference of the (usually inferred) input type.</div></details></div></details>","AsMut<<T as ObjectType>::GlibClassType>","glib::object::ObjectClass"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()