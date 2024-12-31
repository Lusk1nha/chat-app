"use client";

import { Button } from "@/components/ui/button";

import { Path } from "@/shared/enums/Path.enum";
import {
  loginValidation,
  LoginValidationType
} from "@/shared/validations/login-validation";
import { zodResolver } from "@hookform/resolvers/zod";

import { useForm } from "react-hook-form";
import { AuthHeader } from "../_components/auth-header";
import { AuthRedirect } from "../_components/auth-redirect";
import { Separator } from "@/components/ui/separator";
import { FormInput } from "@/components/inputs/form-input";
import { Form } from "@/components/ui/form";
import Link from "next/link";
import { AuthCard } from "../_components/auth-card";

export default function LoginPage() {
  const form = useForm({
    resolver: zodResolver(loginValidation),
    defaultValues: {
      email: "",
      password: ""
    }
  });

  async function handleSubmit(data: LoginValidationType) {
    console.log("submitting form", form);
  }

  return (
    <AuthCard>
      <AuthHeader
        className="mb-6"
        title="Log in to your account"
        subtitle="Sign in to access your chats and messages."
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
            field={{
              type: "password",
              placeholder: "Create a password"
            }}
            control={form.control}
            onLabelPos={<RedirectForgotPassword />}
            required
          />

          <Button type="submit">Login</Button>
        </form>
      </Form>

      <Separator />

      <AuthRedirect href={Path.Register} redirectText="Sign Up">
        No account yet?
      </AuthRedirect>
    </AuthCard>
  );
}

function RedirectForgotPassword() {
  return (
    <p>
      <Link
        className="text-muted-foreground text-sm underline"
        href={Path.Forgot}
      >
        Forgot
      </Link>
    </p>
  );
}
