---
name: good-product
category: taste
description: Apply a product-level design taste distilled from Zed's approach — performance as UX, minimal persistent chrome, restrained visual system, fast paths for repeat users, content-fidelity craft, plain confident copy. Use when defining an app or website's overall look/feel, information architecture, color/type system, or when reviewing a UI for whether it "feels right" at the product level. For micro-interaction and animation-timing polish, use emil-design-eng instead.
---

# Good Product: performance-first, chrome-minimal product design

These are meta-principles, not IDE-specific rules. They apply to any app or
site you build. Each one is a stance to take when a design decision comes
up, not a style to copy literally.

## Initial Response

When first invoked without a specific question, respond only with:

> I'm working from a taste profile distilled from Zed's product design: performance as UX, minimal chrome, restrained visual system, fast paths for repeat users, content-fidelity craft, plain copy. Tell me what you're building or show me what to review.

Do not provide any other information until the user asks a question.

## Core Philosophy

### Performance is a design decision, not an engineering afterthought
Every interaction — click, load, transition — should feel instant. If it
can't be instant, show immediate feedback rather than a blank wait. Treat
perceived latency as a defect regardless of what causes it: a 200ms spinner
delay is a design bug, not just a backend ticket.
- Apply: optimistic UI updates, skeleton states that appear within one
  frame, no "loading..." text with nothing else happening.
- Anti-pattern: disabling a button and waiting for a network round-trip
  before showing any change.

### Progressive disclosure
Default view stays minimal; advanced capability exists but isn't shown
until sought out. Nothing is permanently hidden — power is reachable, not
upfront.
- Apply: collapse secondary settings behind "advanced," surface the 20% of
  actions that cover 80% of use by default.
- Anti-pattern: a settings panel that shows every option flat, with no
  grouping or hierarchy of importance.

### Minimize persistent chrome
Reduce always-on UI (toolbars, nav bars, badges, status text) to what's
truly load-bearing. The user's content occupies the most visual space, not
the app's own controls.
- Apply: contextual toolbars that appear on selection/hover instead of
  living on screen permanently; collapsible panels default closed unless
  they're the primary task.
- Anti-pattern: three rows of icons above the actual content, most of them
  used less than once a session.

### Restrained, consistent iconography
A small, coherent icon/symbol set used the same way everywhere. No
decorative or one-off visual elements that don't carry meaning.
- Apply: one icon library, one stroke weight, one size scale; an icon's
  meaning never changes between screens.
- Anti-pattern: mixing icon styles (filled here, outlined there) or using
  an icon purely for decoration next to text that already says the same
  thing.

### Hierarchy through weight and spacing, not color
Establish importance with type weight, size, and whitespace first. Reserve
color for state and action, not decoration.
- Apply: a heading is bigger/bolder, not just a different hue; whitespace
  separates unrelated groups instead of a rule/border.
- Anti-pattern: five different text colors on one screen, none of them
  meaning "clickable" or "error" consistently.

### Legibility over style, in every theme
Light mode, dark mode, and any other variant are each designed for high
contrast and readability as the primary goal. Style follows legibility, not
the reverse.
- Apply: check contrast ratios per theme independently; don't just invert
  colors and assume it reads the same.
- Anti-pattern: a dark theme that's just the light theme's colors dimmed,
  producing muddy low-contrast text.

### One neutral base, one accent
A restrained neutral palette for structure and content, with a single
consistent accent color reserved for interactive/actionable elements. The
eye should always know where "action" lives.
- Apply: pick one accent, use it for every primary CTA/link/active-state;
  everything else is neutral grays/blacks/whites.
- Anti-pattern: a blue primary button next to a green "save" button next to
  a purple badge, none of them related.

### Give repeat users a fast path
Every core action needs an efficient route for someone doing it often
(shortcut, gesture, quick-action, saved default) — not just the
friendlier, slower, onboarding-oriented default.
- Apply: keyboard shortcuts for power actions, swipe/long-press for mobile,
  a way to skip confirmation dialogs once trust is established.
- Anti-pattern: forcing every user through the same multi-step wizard flow
  every single time, with no memory of prior choices.

### Invest disproportionate craft in the core content type
Identify whatever medium *is* the product (text, images, data, video,
code) and put outsized polish into how it's rendered and presented.
That's where users notice quality first.
- Apply: if it's a reading app, obsess over type rendering and line
  length; if it's a data app, obsess over table density and number
  formatting; if it's visual media, obsess over image fidelity and
  loading behavior.
- Anti-pattern: polished chrome around a content area that's an
  afterthought — cramped, low-contrast, or inconsistently formatted.

### Confident, plain copy — let the product argue for itself
Describe what something does plainly. Prefer demos and real behavior over
superlative marketing language.
- Apply: button labels say the action ("Export CSV," not "Unlock the power
  of your data"); empty states explain what will appear, not hype what's
  coming.
- Anti-pattern: "Supercharge your workflow with blazing-fast AI-powered
  insights!" on a feature that just sorts a list.

## Review Checklist

| Issue | Fix |
| --- | --- |
| Spinner/blank state on every action, even fast ones | Optimistic update or skeleton within one frame |
| All settings/options shown flat, no grouping | Progressive disclosure: common first, advanced collapsed |
| Toolbar/nav visible at all times regardless of relevance | Make contextual, or collapsible with sane default |
| Icon style or meaning changes between screens | Single icon set, single stroke weight, fixed meaning |
| Hierarchy conveyed only by color, not weight/size/space | Rebuild hierarchy with type scale + whitespace first |
| Dark mode is light mode colors inverted/dimmed | Design contrast per theme independently |
| More than one accent color for interactive elements | Collapse to a single accent; rest goes neutral |
| No shortcut/fast path for a frequently repeated action | Add keyboard shortcut, gesture, or remembered default |
| Chrome/decoration more polished than the actual content area | Redirect craft budget to the core content rendering |
| Marketing-voice copy on functional UI text | Rewrite plainly: state the action/result, not the hype |

## When principles conflict
- Progressive disclosure vs. fast path for power users → power users get a
  shortcut that skips the disclosure entirely (e.g. command-driven action),
  novices still see the simple default view.
- Minimal chrome vs. discoverability → contextual reveal (hover/selection)
  beats permanent chrome; don't sacrifice discoverability by hiding an
  action with zero affordance anywhere.
- Single accent color vs. semantic state colors (error/success/warning) →
  accent is for action; red/green/yellow are exempt because they carry
  meaning, not brand.
