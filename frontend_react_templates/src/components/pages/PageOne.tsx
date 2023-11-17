import React from "react";
import axios from "axios";
import "tailwindcss/tailwind.css";

function MasterPage() {
  const registerAPI = async (data: any) => {
    try {
      const response = await axios.post("/register", data);
      // Process response from API call
    } catch (e: any) {
      // Handle error during API call
    }
  };

  const loginAPI = async (data: any) => {
    try {
      const response = await axios.post("/login", data);
      // Process response from API call
    } catch (e: any) {
      // Handle error during API call
    }
  };

  const externalAPI = async () => {
    try {
      const response = await axios.get("https://ipapi.co/json");
      // Process response from API call
    } catch (e: any) {
      // Handle error during API call
    }
  };

  return (
    <div className="w-full">
      <section>
        <div className="px-4 py-12 bg-gradient-to-r from-purple-400 via-pink-500 to-red-500 text-center text-white font-bold">
          <h1 className="text-4xl md:text-6xl lg:text-7xl">
            Catchy title: Track your fitness progress like never before!
          </h1>
          <h2 className="mt-4 text-2xl md:text-4xl lg:text-5xl">
            Subtitle: Our state-of-the-art platform offers all the tools you
            need to reach your goals.
          </h2>
        </div>
        <div className="px-4 py-12 bg-gray-200 text-center text-black font-semibold">
          <p className="text-lg md:text-xl lg:text-2xl mb-6">
            Don't miss your chance to join our growing community and achieve
            your fitness dreams! Sign up today and take the first step towards a
            healthier you.
          </p>
          <button className="rounded-full py-2 px-6 text-white bg-purple-600 hover:bg-purple-700 focus:outline-none">
            Sign up now
          </button>
        </div>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-8 px-4 py-12">
          <div className="text-center">
            <h3 className="text-2xl font-bold">
              Comprehensive progress tracking
            </h3>
            <p className="text-lg mt-4">
              Monitor your workouts, nutrition, and more to ensure you stay on
              track with your fitness journey.
            </p>
          </div>
          <div className="text-center">
            <h3 className="text-2xl font-bold">Personalized workout plans</h3>
            <p className="text-lg mt-4">
              Customize your training routine based on your goals, preferences,
              and fitness level.
            </p>
          </div>
          <div className="text-center">
            <h3 className="text-2xl font-bold">Nutritional guidance</h3>
            <p className="text-lg mt-4">
              Detailed meal plans and grocery lists to fuel your body and
              maximize your results.
            </p>
          </div>
        </div>
      </section>
      <section className="banner_section flex justify-center items-center text-center">
        <h1 className="text-4xl font-bold">Welcome to our Quiz!</h1>
        <p className="text-xl">Test your knowledge and challenge your friends.</p>
      </section>

      <section className="login_section flex justify-center items-center text-center">
        <div className="login_form">
          <h2 className="text-2xl font-bold">Login</h2>
          <form>
            <input type="text" className="input_field" placeholder="Username" />
            <input type="password" className="input_field" placeholder="Password" />
            <button type="submit" className="submit_button">Login</button>
          </form>
        </div>
        <div className="register_form">
          <h2 className="text-2xl font-bold">Register</h2>
          <form>
            <input type="text" className="input_field" placeholder="Username" />
            <input type="password" className="input_field" placeholder="Password" />
            <button type="submit" className="submit_button">Register</button>
          </form>
        </div>
      </section>

      <section className="quiz_preview_section flex justify-center items-center text-center">
        <h2 className="text-2xl font-bold">Quiz Preview</h2>
        <div className="quiz_question">
          <p className="question_text">What is the capital of France?</p>
          <button className="answer_button">Paris</button>
          <button className="answer_button">London</button>
          <button className="answer_button">Berlin</button>
          <button className="answer_button">Madrid</button>
        </div>
        <div className="quiz_question">
          <p className="question_text">Who wrote 'To Kill a Mockingbird'?</p>
          <button className="answer_button">Harper Lee</button>
          <button className="answer_button">J.K. Rowling</button>
          <button className="answer_button">George Orwell</button>
          <button className="answer_button">Ernest Hemingway</button>
        </div>
      </section>
    </div>
  );
}

export default MasterPage;