// src/App.tsx
import { useState } from 'react';

function App() {
  const [jd, setJd] = useState('');
  const [status, setStatus] = useState('');

  const handleSubmit = async () => {
    const file = (document.getElementById('resumeInput') as HTMLInputElement)?.files?.[0];
    if (!file) return alert("Upload a LaTeX resume first.");

    const resumeText = await file.text();

    setStatus("Sending...");

    const response = await fetch('http://localhost:3000/match', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        resume_latex: resumeText,
        job_description: jd,
        user_note: "Focus on tailoring experience to match job description."
      })
    });

    const updated = await response.text();

    const blob = new Blob([updated], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    chrome.downloads.download({ url, filename: 'resume_updated.tex' });

    setStatus("✅ Resume updated and downloaded.");
  };

  return (
    <div style={{ padding: '10px', fontFamily: 'Arial' }}>
      <h3>LaTeX Resume</h3>
      <input type="file" id="resumeInput" accept=".tex" />
      <h3>Job Description</h3>
      <textarea
        rows={8}
        cols={35}
        value={jd}
        onChange={(e) => setJd(e.target.value)}
        placeholder="Paste job description here..."
      />
      <br />
      <button onClick={handleSubmit}>Match Resume</button>
      <p>{status}</p>
    </div>
  );
}

export default App;
