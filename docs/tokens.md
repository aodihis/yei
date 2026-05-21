# Design Tokens

yei uses CSS custom properties as design tokens. All tokens have built-in defaults via CSS `var()` fallbacks inside `yei.css` — no setup required. Override any token in your own CSS to theme the library.

## Usage

```css
@import "tailwindcss";
@import "./yei.css";

/* Override tokens after the import */
:root {
  --primary: oklch(0.5 0.2 250);
  --radius:  0.375rem;
}
```

## Token reference

| Token | Default | Tailwind class | Description |
|-------|---------|----------------|-------------|
| `--background` | `oklch(1 0 0)` | `bg-background`, `text-background` | Page / surface background |
| `--foreground` | `oklch(0.145 0 0)` | `bg-foreground`, `text-foreground` | Primary text colour |
| `--primary` | `oklch(0.205 0 0)` | `bg-primary`, `text-primary` | Primary action colour |
| `--primary-foreground` | `oklch(0.985 0 0)` | `text-primary-foreground` | Text on primary backgrounds |
| `--secondary` | `oklch(0.97 0 0)` | `bg-secondary`, `text-secondary` | Secondary surface colour |
| `--secondary-foreground` | `oklch(0.205 0 0)` | `text-secondary-foreground` | Text on secondary backgrounds |
| `--muted` | `oklch(0.97 0 0)` | `bg-muted`, `text-muted` | Muted / subtle surface colour |
| `--muted-foreground` | `oklch(0.556 0 0)` | `text-muted-foreground` | Subtle / placeholder text |
| `--danger` | `oklch(0.577 0.245 27.325)` | `bg-danger`, `text-danger` | Destructive / error colour |
| `--on-danger` | `oklch(0.985 0 0)` | `text-on-danger` | Text on danger backgrounds |
| `--success` | `oklch(0.527 0.154 150)` | `bg-success`, `text-success` | Success / positive colour |
| `--warning` | `oklch(0.769 0.188 70)` | `bg-warning`, `text-warning` | Warning colour |
| `--field` | `oklch(0.97 0 0)` | `bg-field` | Form field background |
| `--border` | `oklch(0.922 0 0)` | `border-border` | Border colour |
| `--focus` | `oklch(0.708 0 0)` | `ring-focus` | Focus ring colour |
| `--radius` | `0.625rem` | — | Base border radius (used directly in CSS, not via Tailwind) |

## Dark mode example

```css
:root {
  --background:         oklch(0.145 0 0);
  --foreground:         oklch(0.985 0 0);
  --primary:            oklch(0.985 0 0);
  --primary-foreground: oklch(0.205 0 0);
  --secondary:          oklch(0.269 0 0);
  --secondary-foreground: oklch(0.985 0 0);
  --muted:              oklch(0.269 0 0);
  --muted-foreground:   oklch(0.708 0 0);
  --danger:             oklch(0.704 0.191 22.216);
  --on-danger:          oklch(0.985 0 0);
  --field:              oklch(0.205 0 0);
  --border:             oklch(0.269 0 0);
  --focus:              oklch(0.556 0 0);
}
```

## How defaults work

Tokens are set as CSS `var()` fallbacks inside `yei.css`:

```css
@theme inline {
  --color-primary: var(--primary, oklch(0.205 0 0));
}
```

If `--primary` is not defined anywhere, the fallback `oklch(0.205 0 0)` is used automatically. Define `--primary` in your `:root` to override it — no other changes needed.
