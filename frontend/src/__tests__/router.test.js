import { describe, it, expect, beforeEach, vi } from "vitest";

describe("router guard", () => {
  let localStorageMock;

  beforeEach(() => {
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

  it("allows access when authenticated", async () => {
    localStorageMock.token = "fake-token";
    const { default: router } = await import("../router/index.js");
    await router.push({ name: "Home" }).catch(() => {});
    expect(router.currentRoute.value.name).toBe("Home");
  });

  it("redirects logged-in user away from login", async () => {
    localStorageMock.token = "fake-token";
    const { default: router } = await import("../router/index.js");
    await router.push({ name: "Login" }).catch(() => {});
    expect(router.currentRoute.value.name).toBe("Home");
  });
});
