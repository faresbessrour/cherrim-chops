class Automata:
    size = 8
    def __init__(pos_unlocker_x1, pos_unlocker_x2, pos_unlocker_x3):
        self.pos_unlocker_x1 = pos_unlocker_x1
        self.pos_unlocker_x2 = pos_unlocked_x2
        self.pos_unlocker_x3 = pos_unlocker_x3

    def mov(pos_unlocker, size, offset = 0):
        pos_unlocker = (pos_unlocker + 1) % size + offset
        return pos_unlocker
    
    def mov_x1():
        return mov(self.pos_unlocker_x1, self.size)

    def mov_x2():
        return mov(self.pos_unlocker_x2, self.size/2)

    def mov_x3():
        return mov(self.pos_unlocker_x3, self.size/2, self.size/2)


    
