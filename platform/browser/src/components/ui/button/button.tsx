import { cva, type VariantProps } from "class-variance-authority";
import { forwardRef } from "react";

import { classnames } from "@/lib/utils";

import styles from "./styles.module.css";

type ButtonPrimitiveProps = React.ComponentPropsWithoutRef<"button">;
type ButtonVariantProps = VariantProps<typeof buttonVariants>;
type Ref = React.ElementRef<"button">;

export interface ButtonProps extends ButtonPrimitiveProps, ButtonVariantProps {
  isLoading?: boolean;
}

export const Button = forwardRef<Ref, ButtonProps>(
  ({ className, variant, size, children, ...props }, ref) => {
    return (
      <button
        className={classnames(buttonVariants({ variant, size, className }))}
        ref={ref}
        {...props}
      >
        {children}
      </button>
    );
  },
);

Button.displayName = "Button";

export const buttonVariants = cva(styles.btn, {
  variants: {
    variant: {
      primary: styles["btn-primary"],
      secondary: styles["btn-secondary"],
      outline: styles["btn-outline"],
      ghost: styles["btn-ghost"],
      link: styles["btn-link"],
    },
    size: {
      xs: styles["btn-xs"],
      sm: styles["btn-sm"],
      md: styles["btn-md"],
      lg: styles["btn-lg"],
      xl: styles["btn-xl"],
    },
  },
  defaultVariants: {
    variant: "primary",
    size: "md",
  },
});
