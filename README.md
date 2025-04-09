# Multiverify

Verify Rust code using one or more static verification tools. Turn off verification when not needed.

**Why?** Many static verification tools i.e., Creusot, Prusti, etc., require the user to have a backend installed that cannot be easily installed via [crates.io](crates.io). For example, Prusti requires the Viper backend to be installed and Creusot comes with (and requires) at least one SMT solver (though it supports many). These are not native to the Rust ecosystem and once declared as a dependency on a project (i.e., so the project may be verified), require the tool to be installed, which can be cumbersome.

Multiverify aims to fix this problem by ~~abusing~~ using feature flags. With no features enabled, *Multiverify does nothing.* It does not verify any code and the attributes it defines are no-ops. It is only when the feature flag for a certain verifier is given does Multiverify pass through the information to that verifier. This means that by default, a verifier does not need to be installed on a project that uses Multiverify--only if the project is to be statically verified.
