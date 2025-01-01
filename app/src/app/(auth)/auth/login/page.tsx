"use client";

import { Button } from "@/components/ui/button";

import { Path } from "@/shared/enums/Path.enum";
import {
  loginValidation,
  LoginValidationType
} from "@/shared/validations/login-validation";
import { zodResolver } from "@hookform/resolvers/zod";

import { useForm } from "react-hook-form";
import { AuthHeader } from "../../_components/auth-header";
import { AuthRedirect } from "../../_components/auth-redirect";
import { Separator } from "@/components/ui/separator";
import { FormInput } from "@/components/inputs/form-input";
import { Form } from "@/components/ui/form";
import Link from "next/link";
import { AuthCard } from "../../_components/auth-card";

import { useAuth } from "@/hooks/use-auth";
import { toast } from "sonner";
import { redirect } from "next/navigation";

export default function LoginPage() {
  const { login } = useAuth();

  const form = useForm({
    resolver: zodResolver(loginValidation),
    defaultValues: {
      email: "",
      password: ""
    }
  });

  async function handleSubmit(response: LoginValidationType) {
    const data = loginValidation.parse(response);

    toast("Logging in...", {
      id: "login",
      description: "Please wait while we log you in."
    });

    try {
      toast("Logged in successfully!", {
        id: "login",
        description: "You have been successfully logged in.",
        richColors: true
      });

      await login(data);

      toast("Successfully logged in!", {
        id: "login",
        description: "You have been successfully logged in.",
        richColors: true
      });

      setTimeout(() => {
        redirect(Path.DashboardGroup);
      }, 1000);
    } catch (error: any) {
      toast("An error occurred", {
        id: "login",
        description: error.message,
        richColors: true
      });
    }
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

          <Button type="submit" disabled={form.formState.isSubmitting}>
            Login
          </Button>
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
    <p className="text-muted-foreground text-sm underline hover:text-primary">
      <Link href={Path.Forgot}>Forgot</Link>
    </p>
  );
}
