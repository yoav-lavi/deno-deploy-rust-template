import { assertStrictEquals } from "https://deno.land/std@0.146.0/testing/asserts.ts";
import "../src/index.ts";

Deno.test({
  name: "E2E",
  fn: async () => {
    let response = await fetch("http://localhost:8000");
    assertStrictEquals(await response.text(), "hello world!");
  },
  sanitizeResources: false,
  sanitizeOps: false,
});
