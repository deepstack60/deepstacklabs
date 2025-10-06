# PDA Notes — Program Derived Addresses

- PDAs are deterministic accounts derived from seeds.
- Common vulnerabilities:
  1. Incorrect bump seed handling (privilege escalation)
  2. Unsafe use of find_program_address → authority confusion
  3. Reusing seeds that leak business logic assumptions

