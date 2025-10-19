# RCA: GPT-5 Response Parsing Error

**Date:** 2024-10-19 13:51  
**Status:** In Progress - Diagnosing  

## Error Observed

```
Failed to parse response: error decoding response body: missing field `output_text` at line 72 column 1
```

## What We Know

### ✅ Working
1. API request is being sent (logs show: "=== OpenAI API Request ===")
2. Model name: gpt-5-mini
3. Endpoint: `/v1/responses`
4. Audio extraction completed successfully
5. VTT parsing worked (13 cues)
6. User has GPT-5 access (uses it daily)

### ❌ Failing
1. Response parsing - looking for `output_text` field
2. Error at line 72, column 1 of response

### 🤔 Unknown
1. What is the actual API response structure?
2. Is the API returning an error response?
3. Is the field named differently than documented?
4. Is there a wrapper structure we're missing?

## Expected vs Actual

### Expected Response (per docs)
```rust
struct ResponseApiResponse {
    output_text: String,
}
```

### Actual Response
**Need to capture** - We're not logging the raw response before parsing

## Hypotheses

### Hypothesis 1: Field Name Mismatch
**Probability:** HIGH  
**Evidence:** Parsing error specifically says "missing field `output_text`"  
**Possible causes:**
- Field might be `output` instead of `output_text`
- Field might be `text` 
- Field might be nested: `{ output: { text: "..." } }`
- Field might be `content` or `message`

### Hypothesis 2: Error Response from API
**Probability:** MEDIUM  
**Evidence:** Line 72 column 1 suggests complete response, not partial
**Possible causes:**
- API key issue
- Model access issue
- Request format issue
- API returns error in different structure

### Hypothesis 3: Wrapper Structure
**Probability:** MEDIUM  
**Evidence:** GPT-5 docs might have additional response wrapper  
**Possible causes:**
- Response might be wrapped: `{ response: { output_text: "..." } }`
- Multiple response formats depending on parameters
- Version differences in API

### Hypothesis 4: Documentation Mismatch
**Probability:** LOW  
**Evidence:** User provided docs, but maybe partial or different version  
**Possible causes:**
- Docs are for different API version
- Beta vs production differences
- Missing fields in our struct

## Investigation Steps

### Step 1: Capture Raw Response
**Action:** Add logging to print raw API response before parsing

```rust
let response_text = response.text().await?;
println!("=== RAW API RESPONSE ===");
println!("{}", response_text);

let api_response: ResponseApiResponse = serde_json::from_str(&response_text)?;
```

### Step 2: Check API Error Response
**Action:** Check if API is returning error in response body

### Step 3: Verify Request Format
**Action:** Ensure our request matches GPT-5 docs exactly

### Step 4: Test Field Names
**Action:** Try alternative field names if raw response shows different structure

## Next Actions

1. ⏳ Add raw response logging
2. ⏳ Capture actual API response
3. ⏳ Identify correct field name/structure
4. ⏳ Update ResponseApiResponse struct
5. ⏳ Test fix

## DO NOT

- ❌ Assume GPT-5 doesn't exist
- ❌ Rewrite entire service without seeing actual response
- ❌ Change to different model without understanding issue
- ❌ Make multiple speculative changes at once

## Resolution

**Root Cause:** ✅ IDENTIFIED - Response structure mismatch

The documentation showed:
```json
{ "output_text": "..." }
```

But the actual API returns:
```json
{
  "output": [
    { "type": "reasoning", ... },
    { 
      "type": "message",
      "content": [
        {
          "type": "output_text",
          "text": "[actual JSON here]"
        }
      ]
    }
  ]
}
```

The text is at: `output[1].content[0].text`

**Fix:** Update ResponseApiResponse struct to match actual structure  

---

## Notes

- User confirmed GPT-5 is available and used daily
- Documentation provided by user appears to be official OpenAI docs
- Error is in response parsing, not request sending
