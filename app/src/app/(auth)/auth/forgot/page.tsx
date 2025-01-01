"use client";

import { useForm } from "react-hook-form";
import { AuthCard } from "../../_components/auth-card";
import { AuthHeader } from "../../_components/auth-header";
import {
  forgotValidation,
  ForgotValidation
} from "@/shared/validations/forgot-validation";

import authService from "@/shared/factories/auth-factory";
import { FormInput } from "@/components/inputs/form-input";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import { AuthRedirect } from "../../_components/auth-redirect";
import { Path } from "@/shared/enums/Path.enum";

import { zodResolver } from "@hookform/resolvers/zod";
import { Form } from "@/components/ui/form";
import { toast } from "sonner";

export default function ForgotPage() {
  const form = useForm<ForgotValidation>({
    resolver: zodResolver(forgotValidation),
    defaultValues: {
      email: ""
    }
  });

  async function handleSubmit(data: ForgotValidation) {
    const response = forgotValidation.parse(data);

    try {
      toast("Sending email...", {
        id: "forgot",
        description: "Please wait while we send your email."
      });

      await authService.forgotPassword(response.email);

      toast("Email sent!", {
        id: "forgot",
        description: "An email has been sent to your inbox.",
        richColors: true
      });
    } catch (error) {
      toast("Failed to send email", {
        id: "forgot",
        description: "An error occurred while sending your email.",
        richColors: true
      });
    }
  }

  return (
    <AuthCard>
      <AuthHeader
        className="mb-6"
        title="Forgot your password?"
        subtitle="Enter your email address to reset your password."
      />

      <Form {...form}>
        <form
          className="w-full flex flex-col gap-y-4"
          onSubmit={form.handleSubmit(handleSubmit)}
        >
          <FormInput
            name="email"
            label="Email Address"
            field={{
              type: "email",
              placeholder: "Enter your email address"
            }}
            control={form.control}
            required
          />

          <Button
            type="submit"
            className="w-full"
            disabled={
              form.formState.isSubmitting || form.formState.isSubmitSuccessful
            }
          >
            Reset Password
          </Button>
        </form>
      </Form>

      <Separator />

      <AuthRedirect href={Path.Login} redirectText="Back to login">
        Remember your password?
      </AuthRedirect>
    </AuthCard>
  );
}
