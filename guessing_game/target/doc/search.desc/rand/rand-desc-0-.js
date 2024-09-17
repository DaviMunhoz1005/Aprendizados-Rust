searchState.loadedDescShard("rand", 0, "Utilities for random number generation\nCodes at or above this point can be used by users to …\nA marker trait used to indicate that an <code>RngCore</code> or …\nError type of random number generators\nTypes which may be filled with random data\nCodes below this point represent OS Errors (i.e. positive …\nAn automatically-implemented extension trait on <code>RngCore</code> …\nThe core of a random number generator.\nSeed type, which is restricted to types …\nA random number generator that can be explicitly seeded.\nRetrieve the error code, if any.\nGenerating random samples from probability distributions\nFill any type implementing <code>Fill</code> with random data\nFill any type implementing <code>Fill</code> with random data\nFill <code>dest</code> with random data.\nReturns the argument unchanged.\nCreates a new instance of the RNG seeded via <code>getrandom</code>.\nCreate a new PRNG seeded from another <code>Rng</code>.\nCreate a new PRNG using the given seed.\nReturn a random value supporting the <code>Standard</code> distribution.\nReturn a random value supporting the <code>Standard</code> distribution.\nReturn a bool with a probability <code>p</code> of being true.\nReturn a bool with a probability <code>p</code> of being true.\nGenerate a random value in the given range.\nGenerate a random value in the given range.\nReturn a bool with a probability of <code>numerator/denominator</code> …\nReturn a bool with a probability of <code>numerator/denominator</code> …\nReference the inner error (<code>std</code> only)\nCalls <code>U::from(self)</code>.\nConstruct from any type supporting <code>std::error::Error</code>\nReturn the next random <code>u32</code>.\nReturn the next random <code>u64</code>.\nConvenience re-export of common members\nGenerates a random value using the thread-local random …\nExtract the raw OS error code (if this error came from the …\nRandom number generators and adapters\nSample a new value, using the given distribution.\nSample a new value, using the given distribution.\nCreate an iterator that generates values using the given …\nCreate an iterator that generates values using the given …\nCreate a new PRNG using a <code>u64</code> seed.\nSequence-related functionality\nUnwrap the inner error (<code>std</code> only)\nRetrieve the lazily-initialized thread-local random number …\nFill self with random data\nFill any type implementing <code>Fill</code> with random data\nFill any type implementing <code>Fill</code> with random data\nFill <code>dest</code> entirely with random data.\nAll items in the provided weight collection are zero.\nSample a <code>u8</code>, uniformly distributed over ASCII letters and …\nThe Bernoulli distribution.\nError type returned from <code>Bernoulli::new</code>.\nAn iterator that generates random values of <code>T</code> with …\nA distribution of values of type <code>S</code> derived from the …\n<code>String</code> sampler\nTypes (distributions) that can be used to create a random …\n<code>p &lt; 0</code> or <code>p &gt; 1</code>.\nA weight is either less than zero, greater than the …\nThe provided weight collection contains no items.\nA distribution to sample floating point numbers uniformly …\nA distribution to sample floating point numbers uniformly …\nA distribution to sample items uniformly from a slice.\nA generic random value distribution, implemented for many …\nToo many weights are provided (length greater than <code>u32::MAX</code>…\nSample values uniformly between two bounds.\nError type returned from <code>WeightedIndex::new</code>.\nA distribution using weighted sampling of discrete items\nAppend <code>len</code> random chars to <code>string</code>\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nConstruct a new <code>Bernoulli</code> with the probability of success …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCreate a distribution of values of ‘S’ by mapping the …\nCreate a distribution of values of ‘S’ by mapping the …\nConstruct a new <code>Bernoulli</code> with the given probability of …\nCreate a new <code>Slice</code> instance which samples uniformly from …\nCreates a new a <code>WeightedIndex</code> <code>Distribution</code> using the values\nGenerate a random value of <code>T</code>, using <code>rng</code> as the source of …\nCreate an iterator that generates random values of <code>T</code>, …\nCreate an iterator that generates random values of <code>T</code>, …\nGenerate a <code>String</code> of <code>len</code> random chars\nGenerate a <code>String</code> of <code>len</code> random chars\nA distribution uniformly sampling numbers within a given …\nUpdate a subset of weights, without changing the number of …\nWeighted index sampling\nHelper trait similar to <code>Borrow</code> but implemented only for …\nRange that supports generating a single sample efficiently.\nHelper trait for creating objects using the correct …\nThe <code>UniformSampler</code> implementation supporting type <code>X</code>.\nSample values uniformly between two bounds.\nThe back-end implementing <code>UniformSampler</code> for <code>char</code>.\nThe back-end implementing <code>UniformSampler</code> for <code>Duration</code>.\nThe back-end implementing <code>UniformSampler</code> for …\nThe back-end implementing <code>UniformSampler</code> for integer types.\nHelper trait handling actual uniform sampling.\nThe type sampled by this implementation.\nImmutably borrows from an owned value. See <code>Borrow::borrow</code>\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCheck whether the range is empty.\nConstruct self, with inclusive lower bound and exclusive …\nCreate a new <code>Uniform</code> instance which samples uniformly from …\nConstruct self, with inclusive bounds <code>[low, high]</code>.\nCreate a new <code>Uniform</code> instance which samples uniformly from …\nSample a value.\nGenerate a sample from the given range.\nSample a single value uniformly from a range with …\nSample a single value uniformly from a range with …\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nA random number generator that retrieves randomness from …\nThe standard RNG. The PRNG algorithm in <code>StdRng</code> is chosen …\nA reference to the thread-local generator\nWrappers / adapters forming RNGs\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nMock random number generator\n<code>ReadRng</code> error type\nAn RNG that reads random bytes straight from any type …\nA wrapper around any PRNG that implements <code>BlockRngCore</code>, …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCreate a new <code>ReadRng</code> from a <code>Read</code>.\nCreate a new <code>ReseedingRng</code> from an existing PRNG, combined …\nReseed the internal PRNG.\nA simple implementation of <code>RngCore</code> for testing purposes.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCreate a <code>StepRng</code>, yielding an arithmetic sequence starting …\nThe element type.\nExtension trait on iterators, providing random sampling …\nAn iterator over multiple slice elements.\nExtension trait on slices, providing random mutation and …\nReturns a reference to one random element of the slice, or …\nChoose one element at random from the iterator.\nChooses <code>amount</code> elements from the slice at random, without …\nCollects <code>amount</code> values at random from the iterator into a …\nCollects values at random from the iterator into a …\nSimilar to <code>choose_multiple</code>, but where the likelihood of …\nReturns a mutable reference to one random element of the …\nChoose one element at random from the iterator.\nSimilar to <code>choose</code>, but where the likelihood of each …\nSimilar to <code>choose_mut</code>, but where the likelihood of each …\nReturns the argument unchanged.\nLow-level API for sampling indices\nCalls <code>U::from(self)</code>.\nShuffle a slice in place, but exit early.\nShuffle a mutable slice in place.\nA vector of indices.\nReturn type of <code>IndexVec::into_iter</code>.\nReturn type of <code>IndexVec::iter</code>.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturn the value at the given <code>index</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConvert into an iterator over the indices as a sequence of …\nReturn result as a <code>Vec&lt;usize&gt;</code>. Conversion may or may not …\nReturns <code>true</code> if the length is 0.\nIterate over the indices as a sequence of <code>usize</code> values\nReturns the number of indices\nRandomly sample exactly <code>amount</code> distinct indices from …\nRandomly sample exactly <code>amount</code> distinct indices from …")