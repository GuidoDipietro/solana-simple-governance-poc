# Simple Governance Example

Two programs: MockProgram and Governance.

MockProgram has a global state which can be modified by either a designed admin, or a PDA from the Governance program.

Tests are provided with examples using this.

# Advantages

By using a Governance schema like this, the impact to the original program is minimal.

Say MockProgram existed beforehand; if we were to integrate this to a custom Governance program, the only needed change would be to register the Governance PDA as a valid "admin" and that would be it.

# Disclaimer

This entire repo was coded in 25 minutes. Don't expect the best practices, this is just a POC.
