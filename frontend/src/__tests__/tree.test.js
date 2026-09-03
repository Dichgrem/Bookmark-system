import { describe, it, expect } from "vitest";
import { buildTree } from "../utils/tree.js";

describe("buildTree", () => {
  it("returns empty array for empty input", () => {
    expect(buildTree([])).toEqual([]);
  });

  it("builds flat list", () => {
    const items = [
      { id: 1, name: "A", parentId: null, sortOrder: 0 },
      { id: 2, name: "B", parentId: null, sortOrder: 0 },
    ];
    const tree = buildTree(items);
    expect(tree).toHaveLength(2);
    expect(tree[0].name).toBe("A");
    expect(tree[0].children).toEqual([]);
  });

  it("builds nested tree", () => {
    const items = [
      { id: 1, name: "Root", parentId: null, sortOrder: 0 },
      { id: 2, name: "Child", parentId: 1, sortOrder: 0 },
    ];
    const tree = buildTree(items);
    expect(tree).toHaveLength(1);
    expect(tree[0].children).toHaveLength(1);
    expect(tree[0].children[0].name).toBe("Child");
  });

  it("sorts by sortOrder", () => {
    const items = [
      { id: 1, name: "B", parentId: null, sortOrder: 2 },
      { id: 2, name: "A", parentId: null, sortOrder: 1 },
    ];
    const tree = buildTree(items);
    expect(tree[0].name).toBe("A");
    expect(tree[1].name).toBe("B");
  });

  it("preserves extra fields", () => {
    const items = [
      { id: 1, name: "X", parentId: null, sortOrder: 0, extra: 42 },
    ];
    const tree = buildTree(items);
    expect(tree[0].extra).toBe(42);
  });
});
