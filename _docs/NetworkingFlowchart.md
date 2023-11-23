# Networking

```mermaid
flowchart

InitialState[[InitialState]] --> IsIAC?{Is\nIAC\nCommand}
IsIAC? -- Yes --> IACState[[IACState]]
IsIAC? -- No --> CollectingText[[CollectingText]]
CollectingText --> IsIAC2?{Is\nIAC\nCommand}
```
