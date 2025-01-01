import { z } from "zod";

export const signupValidation = z
  .object({
    email: z.string().email("Invalid email address"),
    password: z.string().min(8, "Password must be at least 8 characters"),
    confirmPassword: z.string().min(8, "Password must be at least 8 characters")
  })
  .refine(
    (data) => {
      return data.confirmPassword === data.password;
    },
    {
      message: "Passwords do not match",
      path: ["confirmPassword"]
    }
  );

export type SignupValidationType = z.infer<typeof signupValidation>;
