import { beforeEach, describe, expect, it, vi } from "vitest";
import { invoke } from "@tauri-apps/api/core";

import { CommandError, invokeCommand, toCommandError } from "./commands";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

const mockedInvoke = vi.mocked(invoke);

describe("invokeCommand", () => {
  beforeEach(() => {
    mockedInvoke.mockReset();
  });

  it("devuelve la respuesta del comando", async () => {
    mockedInvoke.mockResolvedValue({ schemaVersion: 1 });

    await expect(invokeCommand("get_app_status")).resolves.toEqual({
      schemaVersion: 1,
    });
    expect(mockedInvoke).toHaveBeenCalledWith("get_app_status", undefined);
  });

  it("mapea un error con código, mensaje y campo", async () => {
    mockedInvoke.mockRejectedValue({
      code: "validation",
      message: "El costo no es válido.",
      field: "totalCost",
    });

    await expect(invokeCommand("create_service")).rejects.toMatchObject({
      name: "CommandError",
      code: "validation",
      message: "El costo no es válido.",
      field: "totalCost",
    });
  });

  it("normaliza errores desconocidos", () => {
    const error = toCommandError(new Error("fallo"));

    expect(error).toBeInstanceOf(CommandError);
    expect(error.code).toBe("unknown");
  });
});
