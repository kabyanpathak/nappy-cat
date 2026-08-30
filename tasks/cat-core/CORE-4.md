# CORE-4: Ecosystem Rules & 5GB Quota Guard Engine

**Epic**: `CAT-CORE-EPIC` (The Shared Foundation)  
**Component**: `crates/cat-core` (Business Logic & Quota Guard)  
**Priority**: High  
**Story Points**: 3 SP  

---

## 🎯 High-Level Goal & System Behavior
Implement the software-enforced **5GB Quota Guard**. 

Because Google Drive accounts provide large storage pools (e.g. 5TB), the ecosystem must strictly enforce a hard 5GB boundary per shared vault folder. The Quota Guard maintains remote state in `_meta/quota.json`, intercepts upload requests *before* network byte transfer occurs, rejects violating payloads, and provides functional auditing tools to recalculate usage.

---

## 🧭 Architectural Milestones
*   [ ] **1. Quota State Schema & Remote Sync**: Define the `QuotaInfo` domain model and methods to fetch/persist state in `_meta/quota.json` on Google Drive.
*   [ ] **2. Pre-Flight Quota Validator**: Implement the pre-upload validation guard ($	ext{used} + 	ext{incoming} \le 5	ext{GB}$) that short-circuits with a custom error before initiating file transfer.
*   [ ] **3. Functional Usage Recalculator**: Write functional iterator pipelines (`.iter().map().sum()`) to audit and recalculate total storage usage across file collections in case of state desynchronization.
*   [ ] **4. State Commit Pipeline**: Implement atomic post-upload increments and post-delete decrements on quota metadata.

---

## 🔒 Invariants & Mathematical Rules
*   **Hard Cap Formula**:
    $$	ext{used\_bytes} + 	ext{incoming\_file\_bytes} \le 5{,}368{,}709{,}120 	ext{ bytes (5 GB)}$$
*   **Short-Circuit Guarantee**: Violating uploads must be aborted immediately without allocating network buffers.

---

## 📚 Documentation & Reference
*   **Rust Functional Iterators**: [https://doc.rust-lang.org/book/ch13-00-functional-features.html](https://doc.rust-lang.org/book/ch13-00-functional-features.html)
*   **Monadic Error Handling (`Result`)**: [https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
*   **`serde_json` Crate**: [https://docs.rs/serde_json/latest/serde_json/](https://docs.rs/serde_json/latest/serde_json/)