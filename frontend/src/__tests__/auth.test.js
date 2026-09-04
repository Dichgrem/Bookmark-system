import { describe, it, expect, beforeEach, vi } from "vitest";
import { isTokenValid } from "../utils/auth.js";
import { LS_TOKEN } from "../utils/constants.js";

function b64url(obj) {
  return btoa(JSON.stringify(obj))
    .replace(/\+/g, "-")
    .replace(/\//g, "_")
    .replace(/=+$/, "");
}

function makeToken(exp) {
  return `${b64url({ alg: "HS256", typ: "JWT" })}.${b64url({
    sub: 1,
    exp,
  })}.signature`;
}

describe("isTokenValid", () => {
  let storage;

  beforeEach(() => {
    storage = {};
    vi.stubGlobal("localStorage", {
      getItem: vi.fn((key) => storage[key] ?? null),
      removeItem: vi.fn((key) => {
        delete storage[key];
      }),
    });
  });

  it("returns false when no token", () => {
    expect(isTokenValid()).toBe(false);
  });

  it("returns true for unexpired token", () => {
    storage[LS_TOKEN] = makeToken(Math.floor(Date.now() / 1000) + 3600);
    expect(isTokenValid()).toBe(true);
  });

  it("returns false for expired token", () => {
    storage[LS_TOKEN] = makeToken(Math.floor(Date.now() / 1000) - 3600);
    expect(isTokenValid()).toBe(false);
  });

  it("returns false for malformed token", () => {
    storage[LS_TOKEN] = "not-a-jwt";
    expect(isTokenValid()).toBe(false);
  });

  it("returns false when payload has no exp", () => {
    storage[LS_TOKEN] = `${b64url({ alg: "HS256" })}.${b64url({ sub: 1 })}.sig`;
    expect(isTokenValid()).toBe(false);
  });
});
