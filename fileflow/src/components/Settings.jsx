// import { useState } from 'react';

function Settings() {
    return (
        <div className="bg-white rounded-md p-8 mx-auto">
            <h1 className="text-2xl font-medium">Settings</h1>
            <div className="felx">
                <label>Start server on launch</label>
                <input className="mx-5" type="checkbox"/>
            </div>
        </div>
    );
}

export default Settings;
