# GraphQL Strategy Proposal Confluence Page Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Create a proposal page in Confluence standardizing the use of Apollo Client primitives and fragment colocation.

**Architecture:** Use the `mcp_Atlassian-Rovo_createConfluencePage` tool to publish the drafted content (Markdown) under the specified parent ID and space.

**Tech Stack:** Confluence Cloud API (via MCP), Markdown.

---

### Task 1: Prepare Confluence Page Content

**Files:**
- Create: `temp_graphql_proposal.md`

- [ ] **Step 1: Write the content to a temporary file**
   
```markdown
# Standardizing GraphQL Strategy: Apollo Client & Fragment Colocation

## 1. Objective
Standardize the GraphQL data fetching pattern across the codebase by moving away from external `.gql` files and `@generated/graphql` hooks. Instead, adopt inline `gql` templates from `@apollo/client` and enforce **Fragment Colocation**.

## 2. Rationale
### Why move away from `.gql` and `@generated/graphql` hooks?
* **Separation of Concerns vs. Fragmentation:** Physical distance between UI logic and data dependencies.
* **Boilerplate & Indirection:** Generated hooks add an abstraction layer that obscures field tracing.
* **Stale Dependencies:** Risk of over-fetching when children components change but parent queries aren't updated.

### Benefits of Apollo Client Primitives + Colocation
* **Encapsulation:** Components explicitly declare their data needs via fragments.
* **DX:** UI and data requirements updated in one place.
* **Maintainability:** Automatic cleanup of data requirements when components are removed.

## 3. Implementation Examples

### A. Fragment Definition (Child Component)
```javascript
// VoteButtons.jsx
import { gql } from '@apollo/client';

export const VOTE_BUTTONS_FRAGMENT = gql`
  fragment VoteButtonsFragment on FeedEntry {
    score
    vote {
      choice
    }
  }
`;
```

### B. Fragment Composition (Intermediate Component)
```javascript
// FeedEntry.jsx
import { gql } from '@apollo/client';
import { VOTE_BUTTONS_FRAGMENT } from './VoteButtons';

export const FEED_ENTRY_FRAGMENT = gql`
  fragment FeedEntryFragment on FeedEntry {
    id
    commentCount
    ...VoteButtonsFragment
  }
  ${VOTE_BUTTONS_FRAGMENT}
`;
```

### C. Root Query Execution (Page/Container)
```javascript
// FeedPage.jsx
import { gql, useQuery } from '@apollo/client';
import { FEED_ENTRY_FRAGMENT } from './FeedEntry';

const GET_FEED = gql`
  query GetFeed {
    feed {
      id
      ...FeedEntryFragment
    }
  }
  ${FEED_ENTRY_FRAGMENT}
`;
```

## 4. Visual Hierarchy (Roll-up Pattern)
```mermaid
graph TD
    subgraph "Root Query (FeedPage.jsx)"
        Q[GET_FEED Query]
        Q -->|Spreads| FEF
    end

    subgraph "Parent Component (FeedEntry.jsx)"
        FE[FeedEntry Component]
        FEF[FeedEntryFragment]
        FEF -->|Spreads| VBF
        FE --- FEF
    end

    subgraph "Child Component (VoteButtons.jsx)"
        VB[VoteButtons Component]
        VBF[VoteButtonsFragment]
        VB --- VBF
    end
```

## 5. References
* [Apollo Client: Queries](https://www.apollographql.com/docs/react/v3/data/queries)
* [Apollo Client: Colocating Fragments](https://www.apollographql.com/docs/react/v3/data/fragments#colocating-fragments)
```

- [ ] **Step 2: Commit temporary file**
   
```bash
git add temp_graphql_proposal.md
git commit -m "docs: temporary GraphQL proposal content"
```

---

### Task 2: Create the Confluence Page

**Files:**
- Remote: Confluence Page

- [ ] **Step 1: Execute `createConfluencePage`**
   
   Use `mcp_Atlassian-Rovo_createConfluencePage`.

- [ ] **Step 2: Record the new page ID**
   
   Verify the response contains the new ID.

---

### Task 3: Cleanup

- [ ] **Step 1: Remove temporary file**
   
```bash
rm temp_graphql_proposal.md
git add temp_graphql_proposal.md
git commit -m "docs: cleanup temporary proposal file"
```
