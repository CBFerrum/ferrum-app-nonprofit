import { describe, expect, it } from "vitest";
import { every, sum } from "./Home";

describe("figures", () => {
  it("adds decimals the list returns as text", () => {
    expect(sum([{ amount: "100.50" }, { amount: "20" }, { amount: null }], "amount")).toBe(120.5);
    expect(sum([], "amount")).toBe(0);
  });

  it("reads every page, not the first", async () => {
    const pages: Record<string, { rows: { amount: string }[]; next_cursor?: string | null }> = {
      "": { rows: [{ amount: "1" }, { amount: "2" }], next_cursor: "p2" },
      p2: { rows: [{ amount: "3" }], next_cursor: null },
    };
    const seen: (string | undefined)[] = [];
    const get = async (_e: string, _f: unknown[], extra: Record<string, unknown>) => {
      seen.push(extra.cursor as string | undefined);
      return pages[(extra.cursor as string | undefined) ?? ""];
    };
    expect(sum(await every("Donation", [], "amount", get), "amount")).toBe(6);
    expect(seen).toEqual([undefined, "p2"]);
  });
});
