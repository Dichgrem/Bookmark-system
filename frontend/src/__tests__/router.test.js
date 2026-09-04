import { describe, it, expect, beforeEach, vi } from "vitest";

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

describe("router guard", () => {
  let localStorageMock;

  beforeEach(() => {
    vi.resetModules();
    localStorageMock = {};
    vi.stubGlobal("localStorage", {
      getItem: vi.fn((key) => localStorageMock[key] ?? null),
      setItem: vi.fn((key, val) => {
        localStorageMock[key] = val;
      }),
      removeItem: vi.fn((key) => {
        delete localStorageMock[key];
      }),
    });
  });

  it("redirects to login when not authenticated", async () => {
    const { default: router } = await import("../router/index.js");
    await router.push({ name: "Home" }).catch(() => {});
    expect(router.currentRoute.value.name).toBe("Login");
  });

  it("allows access with unexpired token", async () => {
    localStorageMock.token = makeToken(Math.floor(Date.now() / 1000) + 3600);
    const { default: router } = await import("../router/index.js");
    await router.push({ name: "Home" }).catch(() => {});
    expect(router.currentRoute.value.name).toBe("Home");
  });

  it("redirects to login when token expired", async () => {
    localStorageMock.token = makeToken(Math.floor(Date.now() / 1000) - 3600);
    const { default: router } = await import("../router/index.js");
    await router.push({ name: "Home" }).catch(() => {});
    expect(router.currentRoute.value.name).toBe("Login");
  });

  it("redirects logged-in user away from login", async () => {
    localStorageMock.token = makeToken(Math.floor(Date.now() / 1000) + 3600);
    const { default: router } = await import("../router/index.js");
    await router.push({ name: "Login" }).catch(() => {});
    expect(router.currentRoute.value.name).toBe("Home");
  });
});
