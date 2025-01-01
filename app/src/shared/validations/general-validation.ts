import { z } from "zod";

export const fileSchemaValidation = z.object({
  name: z.string(),
  size: z.number(),
  type: z.string(),
  url: z.string().optional(),
  file: z.custom<File>().nullable()
});

export type FileSchemaType = z.infer<typeof fileSchemaValidation>;
