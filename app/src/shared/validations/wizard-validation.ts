import { z } from "zod";
import { fileSchemaValidation } from "./general-validation";

export const wizardValidation = z.object({
  name: z
    .string()
    .min(3, "Name must be at least 3 characters long")
    .max(100, "Name must be at most 100 characters long"),
  bio: z.string(),
  avatar_url: fileSchemaValidation.nullable()
});

export type WizardValidation = z.infer<typeof wizardValidation>;
