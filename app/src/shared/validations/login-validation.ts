import { z } from "zod";

export const loginValidation = z.object({
  email: z.string().email("Invalid email address"),
  password: z.string().min(8, "Password must be at least 8 characters")
});

export type LoginValidationType = z.infer<typeof loginValidation>;
