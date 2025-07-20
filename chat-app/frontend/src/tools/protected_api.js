"use client";

import { NextResponse } from "next/server";

export async function POST(req) {
  const { token } = await req.json();

  if (!token) {
    return NextResponse.json({ error: "Token missing" }, { status: 400 });
  }

  const res = NextResponse.json({ message: "Token stored" });

  res.cookies.set("token", token, {
    httpOnly: true,
    secure: process.env.NODE_ENV === "production",
    sameSite: "strict",
    path: "/",
    maxAge: 60 * 60,
  });

  return res;
}

export async function GET() {
  const cookiesStore = cookies();
  const token = cookiesStore.get("token")?.value;

  if (!token)
    return new Response(JSON.stringify({ error: "Unauthorized" }), {
      status: 401,
    });

  try {
    const decoded = jwt.verify(token, JWT_SECRET);
    return Response.json({ user: decoded });
  } catch {
    return new Response(JSON.stringify({ error: "Invalid token" }), {
      status: 403,
    });
  }
}
