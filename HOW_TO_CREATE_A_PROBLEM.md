### How to create a problem for this site?

- Steps:
  - Read the API guide so you own how the submission process looks like for competitors
  - Create a migration for your problem in /api/migrations examples can be seen there
  - Create a problem statement for it in /admin/statements/<code_name>.md
  - Create a problem support program

#### Problem support program
- Determines test count and samples
- Generates test inputs
- Verifies test outputs

Can be a rust service in the backend or an external microservice.

#### Problem support microservice interface

- IDK yet