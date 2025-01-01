"use client";

import { AuthHeader } from "../../_components/auth-header";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import {
  signupValidation,
  SignupValidationType
} from "@/shared/validations/signup-validation";
import { Form } from "@/components/ui/form";

import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";

import { AuthRedirect } from "../../_components/auth-redirect";
import { FormInput } from "@/components/inputs/form-input";
import { Path } from "@/shared/enums/Path.enum";
import { AuthCard } from "../../_components/auth-card";
import authService from "@/shared/factories/auth-factory";
import { toast } from "sonner";
import { redirect } from "next/navigation";

export default function SignupPage() {
  const form = useForm<SignupValidationType>({
    resolver: zodResolver(signupValidation),
    defaultValues: {
      email: "",
      password: "",
      confirmPassword: ""
    }
  });

  async function handleSubmit(data: SignupValidationType) {
    const response = signupValidation.parse(data);

    try {
      toast("Signing up...", {
        id: "signup",
        description: "Please wait while we sign you up."
      });

      await authService.signup(response);

      toast("Successfully signed up!", {
        id: "signup",
        description: "You have been successfully signed up.",
        richColors: true
      });

      setTimeout(() => {
        redirect(Path.DashboardGroup);
      }, 1000);
    } catch (error) {
      toast("Failed to sign up", {
        id: "signup",
        description: "An error occurred while signing you up.",
        richColors: true
      });

      console.error(error);
    }
  }

  return (
    <AuthCard>
      <AuthHeader
        className="mb-6"
        title="Create Your Account"
        subtitle="Sign up to start organizing your notes and boost your productivity."
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

          <FormInput
            name="password"
            label="Password"
            description="At least 8 characters"
            field={{
              type: "password",
              placeholder: "Create a password"
            }}
            control={form.control}
            required
          />

          <FormInput
            name="confirmPassword"
            label="Confirm Password"
            description="At least 8 characters"
            field={{
              type: "password",
              placeholder: "Confirm your password"
            }}
            control={form.control}
            required
          />

          <Button
            type="submit"
            disabled={
              form.formState.isSubmitting || form.formState.isSubmitSuccessful
            }
          >
            Sign Up
          </Button>
        </form>
      </Form>

      <Separator />

      <AuthRedirect href={Path.Login} redirectText="Log in">
        Already have an account?
      </AuthRedirect>
    </AuthCard>
  );
}
