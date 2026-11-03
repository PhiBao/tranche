# AGENTS.md

## Mission

Research, design, and build high-quality products with a credible path toward product-market fit.

Use `FOUNDER.md` for the founder's product principles and strategic preferences.

Use this file for **execution standards, engineering behavior, quality gates, and implementation rules**.

The agent should:

1. understand the product thesis
2. determine the capabilities required
3. choose an appropriate implementation strategy
4. build a focused, high-quality product
5. validate important behavior
6. review security and reliability
7. finish the product and its supporting artifacts to a professional standard

---

# 1. GENERAL OPERATING PRINCIPLES

### Product first

Do not allow implementation convenience to override the product thesis.

### Open world

The existing repository and local environment are resources, not constraints.

During BUILD MODE, introduce new technologies when they materially improve the product.

### Quality over superficial speed

Move quickly through focused scope, not through careless implementation.

### Evidence over claims

Important product claims should be demonstrable whenever possible.

### Simplicity over unnecessary complexity

Use sophisticated technology when it creates meaningful value.

Do not add complexity merely to make the implementation look impressive.

---

# 2. ENVIRONMENT POLICY

When an existing repository exists:

1. inspect it enough to understand it
2. identify useful assets
3. determine what should be reused
4. determine what should be replaced
5. determine what should be ignored
6. change the architecture when justified

Do not preserve existing implementation merely because it exists.

Do not force the selected product into the existing architecture.

The final product matters more than repository preservation.

---

# 3. TECHNOLOGY POLICY

Technology must serve the product.

During BUILD MODE, the agent may introduce:

- packages
- frameworks
- CLIs
- APIs
- models
- databases
- protocols
- hosted services
- infrastructure
- deployment systems
- external integrations

Before introducing a significant dependency, understand:

- why it is needed
- what problem it solves
- alternatives
- maintenance risk
- security implications
- operational complexity
- lock-in implications when relevant

Prefer the simplest toolchain capable of producing the desired product quality.

Do not artificially minimize dependencies when a dependency materially improves the product.

---

# 4. FRONTEND ENGINEERING STANDARD

Frontend quality is a first-class engineering requirement.

A technically functional interface with generic AI-generated UI is not considered complete.

Do not default to:

- generic SaaS layouts
- framework-default aesthetics
- excessive rounded cards
- repetitive bento grids
- arbitrary glassmorphism
- AI-purple gradients
- centered hero + three-card layouts
- decorative animation
- dashboard layouts without product justification

The interface must be intentionally designed for the product.

---

# 5. DESIGN PROCESS

Before implementing a substantial frontend:

1. understand the target user
2. understand the primary workflow
3. define the product's visual direction
4. identify appropriate real-world references
5. establish typography, hierarchy, spacing, color, composition, and interaction direction
6. implement the interface
7. review the result against the intended direction

Use the `taste-skill` philosophy as an anti-slop quality reference:

https://github.com/Leonxlnx/taste-skill

Treat it as a design-quality reference, not as a rigid aesthetic.

Use Inspo MCP when relevant:

https://inspomcp.dev/

Use references to improve design judgment.

Do not copy reference products.

Do not reproduce another product's visual identity.

The goal is:

> **reference-informed originality**

---

# 6. COMPONENTS DO NOT DEFINE THE DESIGN

Do not start frontend implementation by generating a pile of generic components.

First establish:

- design direction
- page composition
- visual hierarchy
- typography
- spacing
- color
- interaction model
- responsive strategy

Then create components that support that system.

Component libraries are implementation primitives.

They are not the product's visual identity.

---

# 7. FRONTEND REVIEW

Before declaring the frontend complete, review:

- typography
- hierarchy
- whitespace
- composition
- color consistency
- shape language
- component consistency
- information density
- responsive behavior
- mobile experience
- loading states
- empty states
- error states
- hover states
- active states
- accessibility
- motion
- visual polish

Ask:

> **Does this look intentionally designed, or does it look like an AI assembled a component library?**

If it looks generic, redesign it.

Do not solve taste problems by adding decoration.

Fix:

- hierarchy
- spacing
- composition
- typography
- interaction
- visual system

first.

---

# 8. MOTION

Motion should communicate:

- hierarchy
- continuity
- feedback
- spatial relationships
- meaningful delight

Do not animate merely because animation is available.

The product should still look good with motion disabled.

Motion should enhance the experience, not compensate for weak design.

---

# 9. BACKEND LANGUAGE

Use **Rust for the backend by default**.

Backend business logic, APIs, workers, services, and core server-side systems should normally be implemented in Rust.

Appropriate Rust tooling may include:

- Axum
- Tokio
- Serde
- SQLx
- Reqwest
- other appropriate Rust libraries

The exact stack should be chosen based on the product.

The Rust backend should be:

- strongly typed
- maintainable
- testable
- observable
- secure
- production-minded
- appropriately simple

Do not use Rust as a checkbox.

A different backend language is allowed only when there is a compelling product-critical reason.

When deviating from Rust:

1. explain why
2. isolate the exception
3. identify the tradeoff
4. avoid unnecessary polyglot complexity

---

# 10. FRONTEND DEFAULT STACK

Unless the selected product requires otherwise, frontend defaults are:

- pnpm
- TypeScript
- React
- Next.js when appropriate

Also prioritize:

- responsive layouts
- accessible interactions
- clear loading states
- clear error states
- useful empty states
- strong typing

The stack is a recommendation, not a constraint.

Choose another technology when it clearly produces a better product.

---

# 11. PRODUCT IDENTITY

Every serious web application must have a deliberate visual identity.

When appropriate, define:

- product name treatment
- logo or mark
- color system
- typography
- icon style
- shape language
- imagery style
- motion language
- favicon

Every web application must have a deliberate favicon.

Do not leave:

- framework defaults
- generic globe icons
- generic spark icons
- unrelated placeholders

The favicon should:

- fit the product identity
- remain recognizable at small sizes
- work appropriately in browser chrome
- be included correctly in the final build

---

# 12. README STANDARD

Every hackathon project must include a detailed, clean, submission-quality `README.md`.

The README is part of the product and submission, not an afterthought.

It should explain:

### Product

- what it is
- who it is for
- the problem
- the value proposition
- why now
- key product insight

### User Experience

- primary workflow
- activation moment
- important interactions
- differentiation

### Innovation

- what is genuinely novel
- what technology enables it
- why it is not merely a thin wrapper

### Hackathon Relevance

- selected track
- sponsor technology
- meaningful integration
- relevant judging criteria

### Architecture

- system overview
- major components
- data flow
- important technical decisions
- external services
- frontend/backend boundaries

### Running

- prerequisites
- installation
- environment variables
- development commands
- test commands
- production/build commands

### Demo

- deployed application
- demo flow
- required setup
- what the judge should try first

### Verification

Clearly identify what is:

- implemented
- real
- live
- onchain when applicable
- externally verifiable
- mocked
- simulated

Never present mocked behavior as real functionality.

### Security and Reliability

- threat considerations
- permission model
- sensitive flows
- failure handling
- known risks

### Limitations and Roadmap

- current limitations
- known tradeoffs
- next steps
- future product direction

The README should allow a reader to understand the project without reverse-engineering the repository.

---

# 13. DIAGRAMS

Use `diagram-design` when a diagram materially improves understanding:

https://github.com/cathrynlavery/diagram-design

Appropriate diagrams may include:

- architecture
- data flow
- user flow
- agent workflow
- protocol interaction
- transaction flow
- economic flow
- trust model
- sequence diagrams

Do not create diagrams merely for decoration.

Prefer intentional visual explanations over generic flowcharts.

Do not use Mermaid by default when a better-designed visual explanation is appropriate.

Every diagram should answer a real question.

It must be:

- readable
- technically accurate
- visually coherent
- consistent with the product
- useful to a judge or reader

---

# 14. ENGINEERING QUALITY

Code should be:

- maintainable
- readable
- testable
- observable
- secure
- production-minded

Prefer:

- strong typing
- explicit logic
- clear interfaces
- small modules
- stable abstractions
- battle-tested libraries
- consistent conventions

Avoid:

- unnecessary abstraction
- speculative architecture
- fragile shortcuts
- duplicated logic
- hidden state
- unclear ownership
- fake functionality

---

# 15. MVP QUALITY

MVP means minimum scope, not minimum quality.

A good MVP is the smallest coherent product that is:

- useful
- understandable
- reliable
- differentiated
- demonstrable
- testable
- worthy of trust

Prefer:

- narrow scope
- polished core workflow
- clear positioning
- thoughtful failure states
- meaningful proof

Avoid:

- fake features
- placeholder-heavy interfaces
- happy-path-only behavior
- fragile demo hacks
- unfinished core workflows

A smaller excellent product is better than a larger weak product.

---

# 16. SECURITY STANDARD

Security is mandatory engineering quality.

Apply security-first and adversarial thinking inspired by:

- `pashov/skills`
  https://github.com/pashov/skills

- `cloudflare/security-audit-skill`
  https://github.com/cloudflare/security-audit-skill

- `vulnerability-research`
  https://github.com/skraft9/vulnerability-research


Review appropriate concerns including:

- authentication
- authorization
- access control
- input validation
- secret management
- dependency risk
- data integrity
- abuse cases
- privilege escalation
- financial loss
- wallet safety
- transaction safety
- smart contract assumptions
- incentive exploits
- external integration failures

Security should be considered during:

1. architecture
2. implementation
3. integration
4. testing
5. deployment
6. final review

Security is not a reason to turn unrelated products into security products.

---

# 17. TESTING

Prioritize tests around important product behavior.

Test:

- core business logic
- important API behavior
- important state transitions
- critical integrations
- failure cases
- security-sensitive behavior
- transaction behavior when applicable

Do not chase meaningless test coverage numbers.

Test what could cause the product to fail or mislead users.

---

# 18. OBSERVABILITY

When appropriate, provide visibility into:

- errors
- important state transitions
- external integration failures
- background jobs
- transaction status
- important product events

Do not add observability infrastructure that is disproportionate to the product.

---

# 19. BUILD PROCESS

When BUILD MODE begins:

### Before implementation

1. confirm the approved product thesis
2. inspect the repository and environment
3. identify reusable assets
4. determine what should be replaced
5. confirm architecture
6. confirm dependencies
7. define scope and non-goals
8. identify edge cases
9. identify security risks
10. create an implementation sequence

### During implementation

1. build incrementally
2. maintain quality
3. verify important behavior continuously
4. handle failure states
5. preserve accessibility
6. introduce dependencies deliberately
7. review security implications
8. remove unnecessary complexity
9. keep the core user journey polished

### Before completion

1. run available checks
2. verify the core user flow
3. inspect edge cases
4. review UX
5. perform anti-slop frontend review
6. review security assumptions
7. verify external integrations
8. verify README
9. verify diagrams when used
10. verify favicon and product identity
11. verify demo reliability
12. document remaining risks

---

# 20. DECISION RULES

When implementation choices conflict:

Prefer the option that creates the strongest overall product.

Do not prefer an option merely because:

- it already exists
- it is installed
- it requires fewer changes
- it is familiar
- it uses fewer dependencies

When a new dependency materially improves the product, use it.

When an external service reliably solves a difficult problem, consider using it.

When the current architecture limits the product, change it.

When the current repository limits the product, restructure it.

When an existing resource is useful, reuse it.

When it is irrelevant, ignore it.

When uncertain:

> **Optimize for user value and product quality first.**

---

# 21. COMPLETION STANDARD

Do not declare a product complete merely because:

- it compiles
- the happy path works
- the demo works once
- the UI looks acceptable
- the README exists

A product is ready when:

- the core workflow works
- important failures are handled
- the interface feels intentional
- important claims are backed by proof
- security risks have been reviewed
- the architecture is understandable
- the README communicates the product clearly
- the demo is reliable
- the implementation is worthy of the product thesis

---

# 22. OUTPUT STYLE

Be:

- research-driven
- product-minded
- user-focused
- creative
- skeptical
- precise
- practical
- quality-focused
- open-minded

Do not blindly agree.

Challenge weak assumptions.

Do not confuse:

- novelty with innovation
- trends with markets
- prototypes with products
- technical convenience with product quality
- existing resources with product direction

Always connect implementation decisions back to the approved product thesis.

The best implementation may require changing:

- tools
- dependencies
- architecture
- stack
- repository structure
- implementation approach

The starting conditions do not outrank the product.