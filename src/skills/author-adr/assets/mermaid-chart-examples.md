# Mermaid Chart Examples

ADR-specific diagram patterns followed by general mermaid syntax reference.

## ADR Status Transitions

```mermaid
stateDiagram-v2
    [*] --> Proposed
    Proposed --> Accepted : Team agrees
    Proposed --> Rejected : Team rejects
    Accepted --> Deprecated : No longer relevant
    Accepted --> Superseded : Replaced by new ADR
    Deprecated --> [*]
    Superseded --> [*]
    Rejected --> [*]
```

## ADR Decision Process

```mermaid
flowchart TD
    A[Identify Design Issue] --> B{Architecturally Significant?}
    B -->|Yes| C[Elicit Options]
    B -->|No| D[Document Informally]
    C --> E[Evaluate Against Criteria]
    E --> F[Choose Option]
    F --> G[Write ADR]
    G --> H[Peer Review]
    H --> I{ecADR Met?}
    I -->|Yes| J[Accept & Implement]
    I -->|No| G
```

## ADR Relationship Graph

```mermaid
graph LR
    ADR1[ADR-1: Use microservices] --> ADR3[ADR-3: API gateway]
    ADR1 --> ADR4[ADR-4: Service mesh]
    ADR2[ADR-2: Use PostgreSQL] --> ADR5[ADR-5: Read replicas]
    ADR3 -.->|superceded by| ADR6[ADR-6: BFF pattern]
    ADR4 --> ADR7[ADR-7: Observability stack]
```

## ADR Lifecycle (Sequence)

```mermaid
sequenceDiagram
    participant Dev as Developer
    participant Team as Team
    participant Repo as Decision Log

    Dev->>Team: Propose decision (draft ADR)
    Team->>Team: Review & discuss options
    Team->>Dev: Feedback (revise or accept)
    Dev->>Repo: Commit accepted ADR
    Note over Repo: Status: Accepted
    Dev->>Repo: Later: new ADR supercedes
    Note over Repo: Status: Superceded by ADR-N
```

## Comparison Table (Preferred for Option Analysis)

When comparing options in an ADR, prefer markdown tables over diagrams:

| Criteria | Option A | Option B | Option C |
|----------|:--------:|:--------:|:--------:|
| Latency  | ✅ Low   | ⚠️ Med   | ❌ High  |
| Cost     | ❌ High  | ✅ Low   | ✅ Low   |
| Maturity | ✅ Proven| ⚠️ New   | ✅ Proven|
| Fit      | ✅       | ✅       | ⚠️       |

---

## General Mermaid Syntax Reference

## Basic Pie Chart

```mermaid-example
pie title NETFLIX
         "Time spent looking for movie" : 90
         "Time spent watching it" : 10
```

```mermaid-example
pie title What Voldemort doesn't have?
         "FRIENDS" : 2
         "FAMILY" : 3
         "NOSE" : 45
```

## Basic sequence diagram

```mermaid-example
sequenceDiagram
    Alice ->> Bob: Hello Bob, how are you?
    Bob-->>John: How about you John?
    Bob--x Alice: I am good thanks!
    Bob-x John: I am good thanks!
    Note right of John: Bob thinks a long<br/>long time, so long<br/>that the text does<br/>not fit on a row.

    Bob-->Alice: Checking with John...
    Alice->John: Yes... John, how are you?
```

## Basic flowchart

```mermaid-example
graph LR
    A[Square Rect] -- Link text --> B((Circle))
    A --> C(Round Rect)
    B --> D{Rhombus}
    C --> D
```

## Larger flowchart with some styling

```mermaid-example
graph TB
    sq[Square shape] --> ci((Circle shape))

    subgraph A
        od>Odd shape]-- Two line<br/>edge comment --> ro
        di{Diamond with <br/> line break} -.-> ro(Rounded<br>square<br>shape)
        di==>ro2(Rounded square shape)
    end

    %% Notice that no text in shape are added here instead that is appended further down
    e --> od3>Really long text with linebreak<br>in an Odd shape]

    %% Comments after double percent signs
    e((Inner / circle<br>and some odd <br>special characters)) --> f(,.?!+-*ز)

    cyr[Cyrillic]-->cyr2((Circle shape Начало));

     classDef green fill:#9f6,stroke:#333,stroke-width:2px;
     classDef orange fill:#f96,stroke:#333,stroke-width:4px;
     class sq,e green
     class di orange
```

## SequenceDiagram: Loops, alt and opt

```mermaid-example
sequenceDiagram
    loop Daily query
        Alice->>Bob: Hello Bob, how are you?
        alt is sick
            Bob->>Alice: Not so good :(
        else is well
            Bob->>Alice: Feeling fresh like a daisy
        end

        opt Extra response
            Bob->>Alice: Thanks for asking
        end
    end
```

## SequenceDiagram: Message to self in loop

```mermaid-example
sequenceDiagram
    participant Alice
    participant Bob
    Alice->>John: Hello John, how are you?
    loop HealthCheck
        John->>John: Fight against hypochondria
    end
    Note right of John: Rational thoughts<br/>prevail...
    John-->>Alice: Great!
    John->>Bob: How about you?
    Bob-->>John: Jolly good!
```

## Sequence Diagram: Blogging app service communication

```mermaid-example
sequenceDiagram
    participant web as Web Browser
    participant blog as Blog Service
    participant account as Account Service
    participant mail as Mail Service
    participant db as Storage

    Note over web,db: The user must be logged in to submit blog posts
    web->>+account: Logs in using credentials
    account->>db: Query stored accounts
    db->>account: Respond with query result

    alt Credentials not found
        account->>web: Invalid credentials
    else Credentials found
        account->>-web: Successfully logged in

        Note over web,db: When the user is authenticated, they can now submit new posts
        web->>+blog: Submit new post
        blog->>db: Store post data

        par Notifications
            blog--)mail: Send mail to blog subscribers
            blog--)db: Store in-site notifications
        and Response
            blog-->>-web: Successfully posted
        end
    end

```

## A commit flow diagram.

```mermaid-example
gitGraph:
    commit "Ashish"
    branch newbranch
    checkout newbranch
    commit id:"1111"
    commit tag:"test"
    checkout main
    commit type: HIGHLIGHT
    commit
    merge newbranch
    commit
    branch b2
    commit
```

<!--- cspell:ignore Ashish newbranch --->