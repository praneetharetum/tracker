import './App.css'

function App() {
  return (
    <div className="container">
      <h1>Welcome to Tracker!</h1>

      <p>Select a family member:</p>

      <div className="family-icons">
        <button className="family-member" onClick={() => console.log('Mom selected')}>
          <span className="icon">👩</span>
          <span className="label">Mom</span>
        </button>
        <button className="family-member" onClick={() => console.log('Dad selected')}>
          <span className="icon">👨</span>
          <span className="label">Dad</span>
        </button>
        <button className="family-member" onClick={() => console.log('Child selected')}>
          <span className="icon">🧒</span>
          <span className="label">Child</span>
        </button>
      </div>
    </div>
  )
}

export default App
