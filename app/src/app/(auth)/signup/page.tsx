"use client";

import { AuthHeader } from "../_components/auth-header";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import {
  signupValidation,
  SignupValidationType
} from "@/shared/validations/signup-validation";
import { Form } from "@/components/ui/form";

import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";

import { AuthRedirect } from "../_components/auth-redirect";
import { FormInput } from "@/components/inputs/form-input";
import { Path } from "@/shared/enums/Path.enum";
import { AuthCard } from "../_components/auth-card";

export default function SignupPage() {
  const form = useForm({
    resolver: zodResolver(signupValidation),
    defaultValues: {
      email: "",
      password: "",
      confirmPassword: ""
    }
  });

  async function handleSubmit(data: SignupValidationType) {
    console.log("submitting form", form);
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

          <Button type="submit">Sign Up</Button>
        </form>
      </Form>

      <Separator />

      <AuthRedirect href={Path.Login} redirectText="Log in">
        Already have an account?
      </AuthRedirect>
    </AuthCard>
  );
}
