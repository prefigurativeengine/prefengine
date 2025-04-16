# General Project Goals & Process Rules
User – anyone using the engine externally in whatever which way (e.g. dev users are those who will build things like frontend components for a type of organization or modules)



## End-Product Goals:

**Match the form of prefengine with its accordant domain models,** and if a change doesn’t match it, a proposal to change that blueprint must be made.

**Balance of ideal design and development speed.** Basically, at all levels of the software, implement the best design for what is needed, but, in general, avoid making novel and complex designs in favor of ideals that take long to develop and maintain.

**Make prefengine accessible for dev users.** This is both accessibility for the varied humans that will use the engine (e.g. blind or deaf people) but also on ease of use regardless of human interface.

**Focus the outside API for the easiest** (and quickest to make) type of apps first.



## Process Rules:

**For building process, make all non-rust dependencies be auto managed** by the build system. Make the build process in general easy and fast.

**Focus contribution merging around model/architecture.** In basic terms this may mean contributions that aren’t part of the model or architecture may stay unmerged for a while. Also before merging a big feature, make sure that that we have the developer capacity to maintain it in the future.

**Prefer to use libraries rather than custom implementations**, however prefer that they used in a decoupled manner should the library increase compile times too much or incur too much developer capacity to keep support in the future.

**Tend to value API stability over experimentation.** Basically, if some proposed change breaks the API for users (when that API is established), there should be a very good reason for it.

**Talking spaces for working groups should value productivity** as well as sufficient communication.

**Welcome new contributors**. Help new developers find their niche. The DM circle specifically should put in effort, when energy is available, to educate those who are willing to learn more about the theories of social justice that drive the project.

**Conversations need to remain respectful** at all times and always follow the Code of Conduct.

