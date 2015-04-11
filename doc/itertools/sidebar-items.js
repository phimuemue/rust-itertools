initSidebarItems({"mod":[["misc","A module of helper traits and iterators that are not intended to be used directly."]],"macro":[["icompr!","`icompr` as in “iterator comprehension” allows creating a mapped iterator with simple syntax, similar to set builder notation, and directly inspired by Python. Supports an optional filter clause."],["iproduct!","Create an iterator over the “cartesian product” of iterators."],["izip!","**Deprecated: use *Zip::new* instead.**"]],"fn":[["linspace","Return an iterator with `n` elements, where the first element is `a` and the last element is `b`."],["times","Return an iterator with `n` elements, for simple repetition a particular number of times. The iterator yields a counter."],["write","**Deprecated: Use *.set_from()* instead**."]],"enum":[["EitherOrBoth","A value yielded by `ZipLongest`. Contains one or two values, depending on which of the input iterators are exhausted."]],"struct":[["Batching","A “meta iterator adaptor”. Its closure recives a reference to the iterator and may pick off as many elements as it likes, to produce the next iterator element."],["Dedup","Remove duplicates from sections of consecutive identical elements. If the iterator is sorted, all elements will be unique."],["FnMap","Clonable iterator adaptor to map elementwise from `Iterator<A>` to `Iterator<B>`"],["GroupBy","Group iterator elements. Consecutive elements that map to the same key (\"runs\"), are returned as the iterator elements of `GroupBy`."],["ISlice","A sliced iterator."],["Interleave","Alternate elements from two iterators until both are run out"],["Intersperse","An iterator adaptor to insert a particular value between each element of the adapted iterator."],["Linspace","An iterator of a sequence of evenly spaced floats."],["Merge","An iterator adaptor that merges the two base iterators in ascending order. If both base iterators are sorted (ascending), the result is sorted."],["MultiPeek","An Iterator adaptor that allows the user to peek at multiple *.next()* values without advancing itself."],["Product","An iterator adaptor that iterates over the cartesian product of the element sets of two iterators **I** and **J**."],["PutBack","An iterator adaptor that allows putting back a single item to the front of the iterator."],["RcIter","A wrapper for `Rc<RefCell<I>>`, that implements the `Iterator` trait."],["Step","An iterator adaptor that steps a number elements in the base iterator for each iteration."],["Stride","Stride is similar to the slice iterator, but with a certain number of steps (the stride) skipped per iteration."],["StrideMut","StrideMut is like Stride, but with mutable elements."],["Tee","One half of an iterator pair where both return the same elements."],["Times","Iterator to repeat a simple number of times"],["Zip","Create an iterator running multiple iterators in lockstep."],["ZipLongest","An iterator which iterates two other iterators simultaneously"]],"trait":[["Itertools","Extra iterator methods for arbitrary iterators"]]});