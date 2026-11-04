import { describe, expect, it } from "vitest";
import { sum } from "./Home";

describe("figures", () => {
  it("adds decimals the list returns as text", () => {
    expect(sum([{ amount: "100.50" }, { amount: "20" }, { amount: null }], "amount")).toBe(120.5);
    expect(sum([], "amount")).toBe(0);
  });
});
