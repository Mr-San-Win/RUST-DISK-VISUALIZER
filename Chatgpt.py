# Requires: pip install python-chess

import tkinter as tk
from tkinter import ttk, messagebox
import random
import chess

# Premium chess app colors - Professional theme
BG_COLOR = "#0f0f23"           # Deep navy background
HEADER_COLOR = "#ffffff"        # Pure white text
SUBHEADER_COLOR = "#c4c7d0"     # Light silver-blue
ACCENT_COLOR = "#6366f1"        # Indigo accent
ACCENT_HOVER = "#4f46e5"        # Deep indigo hover
BOARD_LIGHT = "#f0d9b5"         # Classic light squares
BOARD_DARK = "#b58863"          # Classic dark squares
SIDEBAR_COLOR = "#1e1b4b"       # Rich dark blue sidebar
CARD_COLOR = "#312e81"          # Deep indigo cards
SUCCESS_COLOR = "#10b981"       # Emerald green
WARNING_COLOR = "#f59e0b"       # Amber
DANGER_COLOR = "#ef4444"        # Red
GOLD_ACCENT = "#fbbf24"         # Gold highlights
SURFACE_COLOR = "#1e1b4b"       # Surface elements

# --- Chess Piece Constants (Unicode) ---
PIECES = {
    'r': '♜', 'n': '♞', 'b': '♝', 'q': '♛', 'k': '♚', 'p': '♟',
    'R': '♖', 'N': '♘', 'B': '♗', 'Q': '♕', 'K': '♔', 'P': '♙'
}

# FEN for the starting position (Standard chess notation)
STARTING_FEN = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"

# --------- Simple Evaluation for AI (material only) ----------
PIECE_VALUES = {
    chess.PAWN: 100, chess.KNIGHT: 320, chess.BISHOP: 330,
    chess.ROOK: 500, chess.QUEEN: 900, chess.KING: 0
}

def evaluate(board: chess.Board, ai_color: bool) -> int:
    """Material evaluation from the perspective of ai_color."""
    score = 0
    for pt, val in PIECE_VALUES.items():
        score += len(board.pieces(pt, ai_color)) * val
        score -= len(board.pieces(pt, not ai_color)) * val
    return score

def ai_move_easy(board: chess.Board) -> chess.Move | None:
    """Random legal move, preferring captures."""
    moves = list(board.legal_moves)
    if not moves:
        return None
    captures = [m for m in moves if board.is_capture(m)]
    return random.choice(captures or moves)

def ai_move_medium(board: chess.Board, ai_color: bool) -> chess.Move | None:
    """1-ply lookahead using material evaluation."""
    best, best_moves = -10**9, []
    for m in board.legal_moves:
        board.push(m)
        s = evaluate(board, ai_color)
        board.pop()
        if s > best:
            best, best_moves = s, [m]
        elif s == best:
            best_moves.append(m)
    return random.choice(best_moves) if best_moves else None

def minimax(board: chess.Board, depth: int, alpha: int, beta: int,
            maximizing: bool, ai_color: bool) -> int:
    if depth == 0 or board.is_game_over():
        return evaluate(board, ai_color)
    if maximizing:
        val = -10**9
        for m in board.legal_moves:
            board.push(m)
            val = max(val, minimax(board, depth-1, alpha, beta,
                                   board.turn == ai_color, ai_color))
            board.pop()
            alpha = max(alpha, val)
            if beta <= alpha:
                break
        return val
    else:
        val = 10**9
        for m in board.legal_moves:
            board.push(m)
            val = min(val, minimax(board, depth-1, alpha, beta,
                                   board.turn == ai_color, ai_color))
            board.pop()
            beta = min(beta, val)
            if beta <= alpha:
                break
        return val

def ai_move_hard(board: chess.Board, ai_color: bool, depth: int = 2) -> chess.Move | None:
    """2-ply minimax (alpha-beta)."""
    best, best_moves = -10**9, []
    for m in board.legal_moves:
        board.push(m)
        score = minimax(board, depth-1, -10**9, 10**9, board.turn == ai_color, ai_color)
        board.pop()
        if score > best:
            best, best_moves = score, [m]
        elif score == best:
            best_moves.append(m)
    return random.choice(best_moves) if best_moves else None


class ChessGUI(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("Sam@el's Chess - Premium Edition")
        try:
            self.state("zoomed")  # Fullscreen on Windows
        except Exception:
            self.attributes("-zoomed", True)  # Fallback for some systems
        self.configure(bg=BG_COLOR)

        # Configure grid weights for responsiveness
        self.grid_rowconfigure(0, weight=1)
        self.grid_columnconfigure(0, weight=1)

        # Create frames dictionary
        self.frames = {}
        for F in (WelcomePage, ModeSelectionPage, PvPPage, HumanVsAIPage, LearnChessPage):
            page_name = F.__name__
            frame = F(parent=self, controller=self)
            self.frames[page_name] = frame
            frame.grid(row=0, column=0, sticky="nsew")

        self.show_frame("WelcomePage")

    def show_frame(self, page_name):
        frame = self.frames[page_name]
        frame.tkraise()


class BasePage(tk.Frame):
    def __init__(self, parent, controller):
        super().__init__(parent, bg=BG_COLOR)
        self.controller = controller
        self.grid_rowconfigure(0, weight=1)
        self.grid_columnconfigure(0, weight=1)

    def create_styled_button(self, parent, text, command, bg_color=ACCENT_COLOR, hover_color=ACCENT_HOVER, 
                             font_size=14, width=None, height=None):
        btn = tk.Button(
            parent, text=text, font=("SF Pro Display", font_size, "bold"),
            bg=bg_color, fg="white", activebackground=hover_color,
            relief="flat", bd=0, cursor="hand2",
            command=command, padx=20, pady=10
        )
        if width:
            btn.configure(width=width)
        if height:
            btn.configure(height=height)

        def on_enter(e):
            btn.configure(bg=hover_color, relief="raised", bd=1)
        def on_leave(e):
            btn.configure(bg=bg_color, relief="flat", bd=0)
        btn.bind("<Enter>", on_enter)
        btn.bind("<Leave>", on_leave)
        return btn

    def create_back_button(self, target="ModeSelectionPage"):
        back_frame = tk.Frame(self, bg=BG_COLOR)
        back_frame.pack(side="bottom", fill="x", padx=30, pady=20) 
        btn = self.create_styled_button(
            back_frame, "← Back", 
            lambda: self.controller.show_frame(target),
            bg_color=SURFACE_COLOR, hover_color=SIDEBAR_COLOR
        )
        btn.pack(side="left")

    def create_home_button(self):
        home_frame = tk.Frame(self, bg=BG_COLOR)
        home_frame.pack(side="top", fill="x", padx=30, pady=10)
        btn = self.create_styled_button(
            home_frame, "🏠 Home",
            lambda: self.controller.show_frame("WelcomePage"),
            bg_color=DANGER_COLOR, hover_color="#dc2626",
            font_size=12
        )
        btn.pack(side="right")


class ChessBoardWidget(tk.Frame):
    """Widget for a centered, 8x8 chessboard including algebraic coordinates."""
    def __init__(self, parent, square_size=90):
        super().__init__(parent, bg=BG_COLOR)
        self.square_size = square_size
        self.board_squares = {} 
        self.piece_labels = {} 
        self.current_board_state = self.fen_to_board(STARTING_FEN)
        self.square_click_callback = None  # set by pages to receive clicks

        # Center the board within this frame
        self.grid_rowconfigure(0, weight=1)
        self.grid_columnconfigure(0, weight=1)

        # Outer frame for board border and coordinates
        self.outer_board_frame = tk.Frame(self, bg="black", bd=1, relief="solid")
        self.outer_board_frame.grid(row=0, column=0, sticky="") 

        self.draw_board()

    def fen_to_board(self, fen):
        """Converts FEN piece placement string to an 8x8 array."""
        board = [["" for _ in range(8)] for _ in range(8)]
        parts = fen.split(' ')[0]
        rows = parts.split('/')
        for r, row_str in enumerate(rows):
            c = 0
            for char in row_str:
                if char.isdigit():
                    c += int(char)
                else:
                    board[r][c] = char
                    c += 1
        return board

    def set_position_from_fen(self, piece_fen: str):
        """Re-render pieces from a board placement FEN."""
        self.current_board_state = self.fen_to_board(piece_fen)
        # clear old pieces
        for lbl in list(self.piece_labels.values()):
            lbl.destroy()
        self.piece_labels.clear()
        # draw pieces on existing squares
        for row in range(8):
            for col in range(8):
                code = self.current_board_state[row][col]
                if not code:
                    continue
                square_name = f"{'abcdefgh'[col]}{8-row}"
                square = self.board_squares[square_name]
                lbl = tk.Label(
                    square, 
                    text=PIECES.get(code),
                    font=("Arial Unicode MS", int(self.square_size * 0.45), "bold"),
                    bg=square.cget("bg"),
                    fg=("black" if code.islower() else "white")
                )
                lbl.pack(expand=True)
                self.piece_labels[square_name] = lbl

    def draw_board(self):
        """Draws the 8x8 board, pieces, and algebraic coordinates."""
        coords_frame = tk.Frame(self.outer_board_frame, bg=BG_COLOR)
        coords_frame.pack(padx=2, pady=2)

        # 1. Top coordinates (Files: a-h)
        top_coords = tk.Frame(coords_frame, bg=BG_COLOR)
        top_coords.pack(pady=(0, 2))
        tk.Label(top_coords, text="", bg=BG_COLOR, width=3).pack(side="left")
        for letter in "abcdefgh":
            tk.Label(
                top_coords, text=letter, font=("Segoe UI", 10, "bold"),
                bg=BG_COLOR, fg=SUBHEADER_COLOR, width=int(self.square_size / 10)
            ).pack(side="left")

        # 2. Main board area
        for row in range(8):
            row_frame = tk.Frame(coords_frame, bg=BG_COLOR)
            row_frame.pack()

            # Left rank coordinate
            tk.Label(
                row_frame, text=str(8-row), font=("Segoe UI", 10, "bold"),
                bg=BG_COLOR, fg=SUBHEADER_COLOR, width=3
            ).pack(side="left")

            # Board squares
            for col in range(8):
                square_color = BOARD_LIGHT if (row + col) % 2 == 0 else BOARD_DARK
                square_name = f"{'abcdefgh'[col]}{8-row}"
                square = tk.Frame(
                    row_frame, 
                    width=self.square_size, 
                    height=self.square_size, 
                    bg=square_color
                )
                square.pack(side="left")
                square.pack_propagate(False) 
                self.board_squares[square_name] = square

                # click binding to notify page
                def on_click(event, sq=square_name):
                    if self.square_click_callback:
                        self.square_click_callback(sq)
                square.bind("<Button-1>", on_click)

                # initial piece
                piece_code = self.current_board_state[row][col]
                if piece_code:
                    lbl = tk.Label(
                        square, 
                        text=PIECES.get(piece_code), 
                        font=("Arial Unicode MS", int(self.square_size * 0.45), "bold"), 
                        bg=square_color, 
                        fg=("black" if piece_code.islower() else "white")
                    )
                    lbl.pack(expand=True)
                    self.piece_labels[square_name] = lbl
                    
            # Right rank coordinate
            tk.Label(
                row_frame, text=str(8-row), font=("Segoe UI", 10, "bold"),
                bg=BG_COLOR, fg=SUBHEADER_COLOR, width=3
            ).pack(side="left")

        # 3. Bottom coordinates (Files: a-h)
        bottom_coords = tk.Frame(coords_frame, bg=BG_COLOR)
        bottom_coords.pack(pady=(2, 0))
        tk.Label(bottom_coords, text="", bg=BG_COLOR, width=3).pack(side="left")
        for letter in "abcdefgh":
            tk.Label(
                bottom_coords, text=letter, font=("Segoe UI", 10, "bold"),
                bg=BG_COLOR, fg=SUBHEADER_COLOR, width=int(self.square_size / 10)
            ).pack(side="left")


class WelcomePage(BasePage):
    def __init__(self, parent, controller):
        super().__init__(parent, controller)
        main_frame = tk.Frame(self, bg=BG_COLOR)
        main_frame.pack(expand=True, fill="both")

        header_frame = tk.Frame(main_frame, bg=BG_COLOR)
        header_frame.pack(fill="x", pady=(80, 40))

        title_label = tk.Label(
            header_frame, 
            text="Sam@el's Chess",
            font=("SF Pro Display", 48, "bold"), 
            bg=BG_COLOR, 
            fg=HEADER_COLOR
        )
        title_label.pack(pady=(0, 10))

        subtitle_label = tk.Label(
            header_frame,
            text="PREMIUM CHESS EXPERIENCE",
            font=("SF Pro Display", 16, "bold"),
            bg=BG_COLOR,
            fg=GOLD_ACCENT
        )
        subtitle_label.pack()

        tagline_label = tk.Label(
            header_frame,
            text="It's now time to relax, time to play, and time to learn…",
            font=("SF Pro Display", 16),
            bg=BG_COLOR,
            fg=SUBHEADER_COLOR
        )
        tagline_label.pack(pady=(20, 0))

        features_frame = tk.Frame(main_frame, bg=BG_COLOR)
        features_frame.pack(pady=40)
        features = ["🎯 Multiple Game Modes", "🤖 Smart AI Opponents", "📚 Interactive Learning", "⚡ Modern Interface"]
        for i in range(0, len(features), 2):
            row_frame = tk.Frame(features_frame, bg=BG_COLOR)
            row_frame.pack(pady=8)
            for j in range(2):
                if i + j < len(features):
                    feature_label = tk.Label(
                        row_frame,
                        text=features[i + j],
                        font=("Segoe UI", 16),
                        bg=BG_COLOR,
                        fg=SUBHEADER_COLOR
                    )
                    feature_label.pack(side="left", padx=50)

        button_frame = tk.Frame(main_frame, bg=BG_COLOR)
        button_frame.pack(pady=60)
        start_btn = self.create_styled_button(
            button_frame, "🚀 START PLAYING", 
            lambda: controller.show_frame("ModeSelectionPage"),
            font_size=20, width=18, height=2
        )
        start_btn.pack()


class ModeSelectionPage(BasePage):
    def __init__(self, parent, controller):
        super().__init__(parent, controller)
        self.create_home_button()

        main_frame = tk.Frame(self, bg=BG_COLOR)
        main_frame.pack(expand=True, fill="both", padx=40, pady=10) 

        header_label = tk.Label(
            main_frame,
            text="Choose Your Game Mode",
            font=("SF Pro Display", 32, "bold"),
            bg=BG_COLOR,
            fg=HEADER_COLOR
        )
        header_label.pack(pady=(10, 20)) 

        cards_container = tk.Frame(main_frame, bg=BG_COLOR)
        cards_container.pack(expand=True, fill="both")
        cards_container.grid_columnconfigure(0, weight=1)

        game_modes = [
            {
                "title": "Player vs Player",
                "description": "Challenge a friend in classic chess battle",
                "icon": "👥",
                "page": "PvPPage",
                "color": SUCCESS_COLOR
            },
            {
                "title": "Human vs AI",
                "description": "Test your skills against intelligent AI",
                "icon": "🤖",
                "page": "HumanVsAIPage", 
                "color": ACCENT_COLOR
            },
            {
                "title": "Learn Chess",
                "description": "Master the game with interactive tutorials",
                "icon": "📚",
                "page": "LearnChessPage",
                "color": WARNING_COLOR
            }
        ]

        for i, mode in enumerate(game_modes):
            self.create_mode_card(cards_container, mode, i) 

    def create_mode_card(self, parent, mode_info, index):
        card_frame = tk.Frame(parent, bg=CARD_COLOR, relief="raised", bd=0)
        card_frame.grid(row=index, column=0, pady=8, padx=60, sticky="ew") 

        inner_frame = tk.Frame(card_frame, bg=CARD_COLOR, bd=1, relief="solid",
                               highlightbackground=GOLD_ACCENT, highlightthickness=1)
        inner_frame.pack(fill="both", expand=True, padx=2, pady=2)

        content_frame = tk.Frame(inner_frame, bg=CARD_COLOR)
        content_frame.pack(fill="both", expand=True, padx=30, pady=15) 

        header_frame = tk.Frame(content_frame, bg=CARD_COLOR)
        header_frame.pack(fill="x", pady=(0, 10))

        icon_label = tk.Label(
            header_frame,
            text=mode_info["icon"],
            font=("SF Pro Display", 36), 
            bg=CARD_COLOR,
            fg=GOLD_ACCENT
        )
        icon_label.pack(side="left")

        text_frame = tk.Frame(header_frame, bg=CARD_COLOR)
        text_frame.pack(side="left", fill="x", expand=True, padx=(20, 0))

        title_label = tk.Label(
            text_frame,
            text=mode_info["title"],
            font=("SF Pro Display", 18, "bold"), 
            bg=CARD_COLOR,
            fg=HEADER_COLOR
        )
        title_label.pack(anchor="w")

        desc_label = tk.Label(
            text_frame,
            text=mode_info["description"],
            font=("SF Pro Text", 12), 
            bg=CARD_COLOR,
            fg=SUBHEADER_COLOR
        )
        desc_label.pack(anchor="w", pady=(5, 0))

        button_frame = tk.Frame(content_frame, bg=CARD_COLOR)
        button_frame.pack(fill="x")

        play_btn = self.create_styled_button(
            button_frame, f"PLAY NOW →",
            lambda: self.controller.show_frame(mode_info["page"]),
            bg_color=mode_info["color"],
            font_size=12, width=12 
        )
        play_btn.pack(side="right")


class PvPPage(BasePage):
    """Now playable: two-click move input validated by python-chess."""
    def __init__(self, parent, controller):
        super().__init__(parent, controller)
        self.create_home_button()

        main_container = tk.PanedWindow(self, orient="horizontal", bg=BG_COLOR, sashwidth=0)
        main_container.pack(fill="both", expand=True, padx=20, pady=10) 

        left_panel = tk.Frame(main_container, bg=SIDEBAR_COLOR, width=400)
        main_container.add(left_panel, minsize=350)

        right_panel = tk.Frame(main_container, bg=BG_COLOR)
        main_container.add(right_panel, minsize=600)

        self.setup_game_controls(left_panel)
        self.create_chessboard_display(right_panel)

        self.create_back_button()

        # game state
        self.pyboard = chess.Board()
        self.selected_from = None
        self.turn_label_var.set("Turn: White to move")
        self.chessboard.set_position_from_fen(self.pyboard.board_fen())
        self.chessboard.square_click_callback = self.on_square_click

    def setup_game_controls(self, panel):
        title_frame = tk.Frame(panel, bg=SIDEBAR_COLOR)
        title_frame.pack(fill="x", pady=(5, 20)) 
        tk.Label(
            title_frame,
            text="⚔️ Player vs Player",
            font=("Arial", 22, "bold"),
            bg=SIDEBAR_COLOR,
            fg=GOLD_ACCENT
        ).pack()

        players_section = tk.LabelFrame(
            panel, text=" Players Setup ", font=("Arial", 14, "bold"),
            bg=SIDEBAR_COLOR, fg=HEADER_COLOR, bd=1, relief="solid",
            highlightbackground=ACCENT_COLOR, highlightthickness=1
        )
        players_section.pack(fill="x", padx=20, pady=20)

        tk.Label(players_section, text="⚪ White Player:",
                 font=("Arial", 12, "bold"), bg=SIDEBAR_COLOR, fg=SUBHEADER_COLOR
        ).pack(anchor="w", padx=15, pady=(15, 5))
        white_entry = tk.Entry(players_section, font=("Arial", 12),
                               bg=SURFACE_COLOR, fg=HEADER_COLOR, insertbackground=HEADER_COLOR,
                               relief="flat", bd=3, highlightbackground=ACCENT_COLOR, highlightthickness=1)
        white_entry.pack(fill="x", padx=15, pady=(0, 15))
        white_entry.insert(0, "Player 1")

        tk.Label(players_section, text="⚫ Black Player:",
                 font=("Arial", 12, "bold"), bg=SIDEBAR_COLOR, fg=SUBHEADER_COLOR
        ).pack(anchor="w", padx=15, pady=(0, 5))
        black_entry = tk.Entry(players_section, font=("Arial", 12),
                               bg=SURFACE_COLOR, fg=HEADER_COLOR, insertbackground=HEADER_COLOR,
                               relief="flat", bd=3, highlightbackground=ACCENT_COLOR, highlightthickness=1)
        black_entry.pack(fill="x", padx=15, pady=(0, 15))
        black_entry.insert(0, "Player 2")

        time_section = tk.LabelFrame(
            panel, text=" Time Control ", font=("Arial", 14, "bold"),
            bg=SIDEBAR_COLOR, fg=HEADER_COLOR, bd=1, relief="solid",
            highlightbackground=ACCENT_COLOR, highlightthickness=1
        )
        time_section.pack(fill="x", padx=20, pady=20)
        for option, color in [("⏰ No Time Limit", SUCCESS_COLOR),
                              ("⚡ 5 Minutes", WARNING_COLOR),
                              ("🕐 10 Minutes", ACCENT_COLOR),
                              ("⏳ 30 Minutes", DANGER_COLOR)]:
            btn = self.create_styled_button(time_section, option, lambda: None, bg_color=color, font_size=13)
            btn.pack(fill="x", pady=8, padx=15)

        self.turn_label_var = tk.StringVar(value="Turn:")
        tk.Label(panel, textvariable=self.turn_label_var,
                 font=("Arial", 14, "bold"), bg=SIDEBAR_COLOR, fg=GOLD_ACCENT
        ).pack(pady=(10, 0))

        start_btn = self.create_styled_button(panel, "🎮 START GAME",
                                              self.reset_game, bg_color=SUCCESS_COLOR,
                                              font_size=18, height=2)
        start_btn.pack(fill="x", padx=20, pady=30)

    def create_chessboard_display(self, panel):
        board_container = tk.Frame(panel, bg=BG_COLOR)
        board_container.pack(expand=True, fill="both", padx=40, pady=10) 
        board_container.grid_rowconfigure(0, weight=1)
        board_container.grid_columnconfigure(0, weight=1)

        board_title = tk.Label(
            board_container,
            text="♔ CHESS BOARD ♛",
            font=("Arial", 24, "bold"),
            bg=BG_COLOR,
            fg=GOLD_ACCENT
        )
        board_title.grid(row=0, column=0, pady=(0, 5), sticky="n") 

        self.chessboard = ChessBoardWidget(board_container, square_size=90) 
        self.chessboard.grid(row=0, column=0, sticky="n") 

    def reset_game(self):
        self.pyboard = chess.Board()
        self.selected_from = None
        self.chessboard.set_position_from_fen(self.pyboard.board_fen())
        self.turn_label_var.set("Turn: White to move")

    def on_square_click(self, sq_name: str):
        # two-click move input
        if self.selected_from is None:
            self.selected_from = sq_name
            return
        move_uci = f"{self.selected_from}{sq_name}"
        self.selected_from = None
        try:
            move = chess.Move.from_uci(move_uci)
            if move in self.pyboard.legal_moves:
                self.pyboard.push(move)
                self.chessboard.set_position_from_fen(self.pyboard.board_fen())
                if self.pyboard.is_game_over():
                    self.turn_label_var.set("Game over")
                    messagebox.showinfo("Game Over", f"Result: {self.pyboard.result()}")
                else:
                    self.turn_label_var.set("Turn: " + ("White" if self.pyboard.turn == chess.WHITE else "Black") + " to move")
        except Exception:
            pass


class HumanVsAIPage(BasePage):
    def __init__(self, parent, controller):
        super().__init__(parent, controller)

        self.create_home_button()

        main_container = tk.PanedWindow(self, orient="horizontal", bg=BG_COLOR, sashwidth=0)
        main_container.pack(fill="both", expand=True, padx=20, pady=20)

        left_panel = tk.Frame(main_container, bg=SIDEBAR_COLOR, width=400)
        main_container.add(left_panel, minsize=350)

        right_panel = tk.Frame(main_container, bg=BG_COLOR)
        main_container.add(right_panel, minsize=600)

        self.setup_ai_controls(left_panel)
        self.create_chessboard_display(right_panel)

        self.create_back_button()

        # game state
        self.ai_level = "easy"           # "easy" | "medium" | "hard"
        self.human_color = chess.WHITE   # set via color buttons
        self.pyboard = chess.Board()
        self.selected_from = None
        self.chessboard.set_position_from_fen(self.pyboard.board_fen())
        self.chessboard.square_click_callback = self.on_square_click

    def setup_ai_controls(self, panel):
        title_frame = tk.Frame(panel, bg=SIDEBAR_COLOR)
        title_frame.pack(fill="x", pady=20)
        tk.Label(
            title_frame,
            text="🤖 Human vs AI",
            font=("Segoe UI", 26, "bold"),
            bg=SIDEBAR_COLOR,
            fg=HEADER_COLOR
        ).pack()

        # Player color selection
        color_section = tk.LabelFrame(
            panel, text=" Choose Your Color ", font=("Segoe UI", 16, "bold"),
            bg=SIDEBAR_COLOR, fg=HEADER_COLOR, bd=2, relief="groove"
        )
        color_section.pack(fill="x", padx=20, pady=20)

        color_frame = tk.Frame(color_section, bg=SIDEBAR_COLOR)
        color_frame.pack(pady=15)

        white_btn = self.create_styled_button(color_frame, "⚪ Play as White",
                                              self.set_human_white,
                                              bg_color=SUCCESS_COLOR, font_size=14)
        white_btn.pack(pady=5, padx=15, fill="x")

        black_btn = self.create_styled_button(color_frame, "⚫ Play as Black",
                                              self.set_human_black,
                                              bg_color=DANGER_COLOR, font_size=14)
        black_btn.pack(pady=5, padx=15, fill="x")

        # AI Difficulty section
        difficulty_section = tk.LabelFrame(
            panel, text=" AI Difficulty Level ", font=("Segoe UI", 16, "bold"),
            bg=SIDEBAR_COLOR, fg=HEADER_COLOR, bd=2, relief="groove"
        )
        difficulty_section.pack(fill="x", padx=20, pady=20)

        def add_level_button(title, lvl, color, desc):
            diff_frame = tk.Frame(difficulty_section, bg=SIDEBAR_COLOR)
            diff_frame.pack(fill="x", padx=15, pady=8)
            btn = self.create_styled_button(diff_frame, title,
                                            lambda l=lvl: self.set_difficulty(l),
                                            bg_color=color, font_size=13)
            btn.pack(fill="x")
            tk.Label(diff_frame, text=desc, font=("Segoe UI", 11),
                     bg=SIDEBAR_COLOR, fg=SUBHEADER_COLOR).pack(pady=(5, 0))

        add_level_button("🥉 Easy → Club Player", "easy", SUCCESS_COLOR, "Perfect for beginners")
        add_level_button("🥈 Medium → National Master", "medium", WARNING_COLOR, "Intermediate challenge")
        add_level_button("🥇 Hard → World Champion", "hard", DANGER_COLOR, "Expert level AI")

        # AI Status
        ai_section = tk.LabelFrame(
            panel, text=" AI Status ", font=("Segoe UI", 16, "bold"),
            bg=SIDEBAR_COLOR, fg=HEADER_COLOR, bd=2, relief="groove"
        )
        ai_section.pack(fill="x", padx=20, pady=20)

        self.ai_status = tk.Label(
            ai_section, text="🤖 AI Ready (Easy)",
            font=("Segoe UI", 16, "bold"), bg=SIDEBAR_COLOR, fg=ACCENT_COLOR
        )
        self.ai_status.pack(pady=20)

        # Challenge AI button
        challenge_btn = self.create_styled_button(
            panel, "⚔️ CHALLENGE AI",
            self.start_ai_game, bg_color=DANGER_COLOR,
            font_size=18, height=2
        )
        challenge_btn.pack(fill="x", padx=20, pady=30)

    def create_chessboard_display(self, panel):
        board_container = tk.Frame(panel, bg=BG_COLOR)
        board_container.pack(expand=True, fill="both", padx=40, pady=10)
        board_container.grid_rowconfigure(0, weight=1)
        board_container.grid_columnconfigure(0, weight=1)

        board_title = tk.Label(
            board_container,
            text="♔ BATTLE ARENA ♛",
            font=("Segoe UI", 28, "bold"),
            bg=BG_COLOR,
            fg=HEADER_COLOR
        )
        board_title.grid(row=0, column=0, pady=(0, 5), sticky="n")

        self.chessboard = ChessBoardWidget(board_container, square_size=90)
        self.chessboard.grid(row=0, column=0, sticky="n")

    # --- Game control helpers ---
    def set_difficulty(self, lvl: str):
        self.ai_level = lvl
        if self.ai_status:
            self.ai_status.config(text=f"🤖 AI Ready ({lvl.title()})")

    def set_human_white(self):
        self.human_color = chess.WHITE
        if self.ai_status:
            self.ai_status.config(text=f"🤖 AI Ready ({self.ai_level.title()}) — You are White")

    def set_human_black(self):
        self.human_color = chess.BLACK
        if self.ai_status:
            self.ai_status.config(text=f"🤖 AI Ready ({self.ai_level.title()}) — You are Black")

    def start_ai_game(self):
        self.pyboard = chess.Board()
        self.selected_from = None
        self.chessboard.set_position_from_fen(self.pyboard.board_fen())
        if self.human_color == chess.BLACK:
            # AI moves first as White
            self.after(200, self.ai_turn)

    def on_square_click(self, sq_name: str):
        # human move only when it's human's turn
        if self.pyboard.is_game_over():
            messagebox.showinfo("Game Over", f"Result: {self.pyboard.result()}")
            return

        if self.pyboard.turn != self.human_color:
            return  # not human's turn

        if self.selected_from is None:
            self.selected_from = sq_name
            return

        move_uci = f"{self.selected_from}{sq_name}"
        self.selected_from = None
        try:
            move = chess.Move.from_uci(move_uci)
            if move in self.pyboard.legal_moves:
                self.pyboard.push(move)
                self.chessboard.set_position_from_fen(self.pyboard.board_fen())
                if self.pyboard.is_game_over():
                    messagebox.showinfo("Game Over", f"Result: {self.pyboard.result()}")
                else:
                    self.after(200, self.ai_turn)
        except Exception:
            pass

    def ai_turn(self):
        if self.pyboard.is_game_over():
            return
        ai_color = not self.human_color
        if self.ai_level == "easy":
            mv = ai_move_easy(self.pyboard)
        elif self.ai_level == "medium":
            mv = ai_move_medium(self.pyboard, ai_color)
        else:
            mv = ai_move_hard(self.pyboard, ai_color, depth=2)

        if mv is None:
            return
        self.pyboard.push(mv)
        self.chessboard.set_position_from_fen(self.pyboard.board_fen())
        if self.pyboard.is_game_over():
            messagebox.showinfo("Game Over", f"Result: {self.pyboard.result()}")
        elif self.ai_status:
            self.ai_status.config(text=f"🤖 AI moved: {mv.uci()}")


class LearnChessPage(BasePage):
    def __init__(self, parent, controller):
        super().__init__(parent, controller)

        # Home button placement (uses grid)
        self.home_frame = tk.Frame(self, bg=BG_COLOR)
        self.home_frame.grid(row=0, column=0, sticky="ne", padx=30, pady=10)
        self.create_home_button_in_frame(self.home_frame)
        
        self.grid_rowconfigure(1, weight=1)
        self.grid_columnconfigure(0, weight=1)

        # Main container frame
        main_frame = tk.Frame(self, bg=BG_COLOR)
        main_frame.grid(row=1, column=0, sticky="nsew", padx=40, pady=0)
        main_frame.grid_columnconfigure(0, weight=1)

        # Header section
        header_frame = tk.Frame(main_frame, bg=BG_COLOR)
        header_frame.grid(row=0, column=0, pady=(0, 20), sticky="ew")

        tk.Label(
            header_frame,
            text="📚 Learn Chess Fundamentals",
            font=("Segoe UI", 36, "bold"),
            bg=BG_COLOR,
            fg=HEADER_COLOR
        ).pack(pady=(0, 10))

        tk.Label(
            header_frame,
            text="Master the 6 pieces with this interactive grid guide",
            font=("Segoe UI", 18),
            bg=BG_COLOR,
            fg=SUBHEADER_COLOR
        ).pack()

        # GRID CONTAINER FOR 3x2 LAYOUT
        cards_container = tk.Frame(main_frame, bg=BG_COLOR)
        cards_container.grid(row=1, column=0, sticky="ew", pady=20)
        cards_container.grid_columnconfigure((0, 1, 2), weight=1)
        
        pieces_data = [
            { "name": "♙ Pawn", "symbol": "♙♟", "description": "The foot soldier. Moves forward one square, captures diagonally forward.", "special": "First move: Can advance two squares • Promotion at end rank", "color": SUCCESS_COLOR },
            { "name": "♖ Rook", "symbol": "♖♜", "description": "The castle. Moves any number of squares horizontally or vertically.", "special": "Castling with king • Powerful in endgames", "color": ACCENT_COLOR },
            { "name": "♘ Knight", "symbol": "♘♞", "description": "The horse. Moves in an 'L' shape: 2 squares in one direction, 1 perpendicular.", "special": "Only piece that can jump over others • Great for forks and tactics", "color": WARNING_COLOR },
            { "name": "♗ Bishop", "symbol": "♗♝", "description": "The diagonal mover. Travels any number of squares along diagonal lines.", "special": "Light or dark squared • Two bishops work well together", "color": "#9C27B0" },
            { "name": "♕ Queen", "symbol": "♕♛", "description": "The most powerful piece. Combines rook and bishop movement patterns.", "special": "Can move in all 8 directions • Most valuable piece", "color": DANGER_COLOR },
            { "name": "♔ King", "symbol": "♔♚", "description": "The most important piece. Moves one square in any direction.", "special": "Must be protected at all costs • Checkmate ends the game", "color": "#FF9800" }
        ]

        for i, piece in enumerate(pieces_data):
            self.create_learning_card(cards_container, piece, i // 3, i % 3)

        # Basic Rules Section
        self.create_basic_rules_section(main_frame)
        
        self.create_back_button_grid(target="ModeSelectionPage")

    def create_home_button_in_frame(self, frame):
        btn = self.create_styled_button(
            frame, "🏠 Home",
            lambda: self.controller.show_frame("WelcomePage"),
            bg_color=DANGER_COLOR, hover_color="#dc2626",
            font_size=12
        )
        btn.pack(side="right")
        
    def create_back_button_grid(self, target="ModeSelectionPage"):
        back_frame = tk.Frame(self, bg=BG_COLOR)
        back_frame.grid(row=2, column=0, sticky="sw", padx=30, pady=20) 
        btn = self.create_styled_button(
            back_frame, "← Back", 
            lambda: self.controller.show_frame(target),
            bg_color=SURFACE_COLOR, hover_color=SIDEBAR_COLOR
        )
        btn.pack(side="left") 

    def create_learning_card(self, parent, piece_info, row, column):
        card_frame = tk.Frame(parent, bg=CARD_COLOR, relief="raised", bd=2)
        card_frame.grid(row=row, column=column, sticky="nsew", padx=10, pady=10)
        parent.grid_rowconfigure(row, weight=1) 
        content_frame = tk.Frame(card_frame, bg=CARD_COLOR)
        content_frame.pack(fill="both", padx=15, pady=15) 
        header_frame = tk.Frame(content_frame, bg=CARD_COLOR)
        header_frame.pack(fill="x", pady=(0, 5)) 

        symbol_label = tk.Label(
            header_frame,
            text=piece_info["symbol"],
            font=("Arial Unicode MS", 32),
            bg=CARD_COLOR,
            fg=HEADER_COLOR
        )
        symbol_label.pack(side="left")

        info_frame = tk.Frame(header_frame, bg=CARD_COLOR)
        info_frame.pack(side="left", fill="x", expand=True, padx=(15, 0))

        name_label = tk.Label(
            info_frame,
            text=piece_info["name"],
            font=("Segoe UI", 16, "bold"),
            bg=CARD_COLOR,
            fg=piece_info["color"]
        )
        name_label.pack(anchor="w")

        desc_label = tk.Label(
            info_frame,
            text=piece_info["description"],
            font=("Segoe UI", 10),
            bg=CARD_COLOR,
            fg=SUBHEADER_COLOR,
            wraplength=200,
            justify="left"
        )
        desc_label.pack(anchor="w", pady=(5, 0))

        special_frame = tk.Frame(content_frame, bg="#333333", relief="groove", bd=1)
        special_frame.pack(fill="x", pady=(10, 0))

        tk.Label(
            special_frame,
            text="• " + piece_info["special"],
            font=("Segoe UI", 10),
            bg="#333333",
            fg=piece_info["color"],
            wraplength=250,
            justify="left",
            padx=10, pady=5
        ).pack(anchor="w")

    def create_basic_rules_section(self, parent):
        rules_header = tk.Frame(parent, bg=BG_COLOR)
        rules_header.grid(row=2, column=0, sticky="ew", pady=(20, 10))
        tk.Label(
            rules_header,
            text="⚔️ Basic Rules Summary",
            font=("Segoe UI", 20, "bold"),
            bg=BG_COLOR,
            fg=HEADER_COLOR
        ).pack()

        rules_container = tk.Frame(parent, bg=BG_COLOR)
        rules_container.grid(row=3, column=0, sticky="ew")
        
        rules = [
            { "title": "🎯 Objective:", "content": "Checkmate your opponent's king (trapped and attacked).", "color": SUCCESS_COLOR },
            { "title": "🔄 Turn Order:", "content": "White moves first, then players alternate turns.", "color": ACCENT_COLOR },
            { "title": "🏰 Special Moves:", "content": "Castling, En passant, and Pawn Promotion.", "color": WARNING_COLOR },
            { "title": "🏁 Game End:", "content": "Checkmate, Stalemate (draw), or agreement.", "color": DANGER_COLOR }
        ]

        rules_summary_frame = tk.Frame(rules_container, bg=BG_COLOR)
        rules_summary_frame.pack(pady=10)
        rules_summary_frame.grid_columnconfigure((0, 1, 2, 3), weight=1)

        for i, rule in enumerate(rules):
            rule_card = tk.Frame(rules_summary_frame, bg=CARD_COLOR, relief="raised", bd=1)
            rule_card.grid(row=0, column=i, padx=5, pady=5, sticky="nsew")
            tk.Label(
                rule_card,
                text=rule["title"] + "\n" + rule["content"],
                font=("Segoe UI", 10),
                bg=CARD_COLOR,
                fg=rule["color"],
                wraplength=150,
                justify="center",
                padx=10, pady=8
            ).pack(expand=True, fill="both")


if __name__ == "__main__":
    app = ChessGUI()
    app.mainloop()
