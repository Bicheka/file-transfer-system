// import { useState } from 'react';

function Settings() {
  return (
    <div className="mx-auto rounded-md bg-white p-8">
      <h1 className="text-2xl font-medium">Settings</h1>
      <div className="felx">
        <label>Start server on launch</label>
        <input className="mx-5" type="checkbox" />
      </div>
      <div>
        <label>Color theme</label>
        <label htmlFor="cars">theme:</label>
        <select name="cars" id="cars">
          <option value="volvo">light</option>
          <option value="saab">Dark</option>
        </select>
      </div>
    </div>
  );
}

export default Settings;
