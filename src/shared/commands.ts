import { invoke } from "@tauri-apps/api/core";

export type CommandErrorPayload = {
  code: string;
  message: string;
  field?: string | null;
};

export class CommandError extends Error {
  readonly code: string;
  readonly field?: string;

  constructor(payload: CommandErrorPayload) {
    super(payload.message);
    this.name = "CommandError";
    this.code = payload.code;
    this.field = payload.field ?? undefined;
  }
}

export function toCommandError(error: unknown): CommandError {
  if (error instanceof CommandError) {
    return error;
  }

  if (typeof error === "object" && error !== null) {
    const payload = error as Partial<CommandErrorPayload>;

    if (typeof payload.code === "string" && typeof payload.message === "string") {
      return new CommandError({
        code: payload.code,
        message: payload.message,
        field: payload.field,
      });
    }
  }

  return new CommandError({
    code: "unknown",
    message: "Ocurrió un error inesperado.",
  });
}

export async function invokeCommand<T>(
  command: string,
  args?: Record<string, unknown>,
): Promise<T> {
  try {
    return await invoke<T>(command, args);
  } catch (error) {
    throw toCommandError(error);
  }
}
