# Character Creation Process

Follow these steps to create your character:

1. Provide your username.
2. Select your preferred pronouns.
3. Choose a city to start in.
4. Confirm your choices.

- If you need to make changes, you can do so before confirming.

```mermaid
flowchart

A[Login Menu] -->|Create Character| B[Provide Character Name]
B -->|Name Provided| C{Name Valid?}
C -->|No| B

C -->|Yes| D{Name Taken?}
D -->|Yes| B

D -->|No| E{Pronouns}
```
