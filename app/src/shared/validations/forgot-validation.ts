import { z } from "zod";

export const forgotValidation = z.object({
  email: z.string().email("Invalid email address")
});

export type ForgotValidation = z.infer<typeof forgotValidation>;
